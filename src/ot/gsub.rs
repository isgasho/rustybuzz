use std::convert::TryFrom;

use ttf_parser::GlyphId;
use ttf_parser::parser::{LazyArray16, Stream, Offset, Offset16, Offsets16};

use crate::ffi;
use crate::buffer::GlyphPropsFlags;
use super::Map;
use super::ggg::Coverage;
use super::layout::{ApplyContext, WouldApplyContext};

#[derive(Clone, Copy, Debug)]
enum SingleSubst<'a> {
    Format1 {
        coverage: Coverage<'a>,
        delta: i16,
    },
    Format2 {
        coverage: Coverage<'a>,
        substitutes: LazyArray16<'a, GlyphId>,
    },
}

impl<'a> SingleSubst<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let format = s.read::<u16>()?;
        Some(match format {
            1 => {
                let offset = s.read::<Offset16>()?.to_usize();
                let coverage = Coverage::parse(data.get(offset..)?)?;
                let delta = s.read::<i16>()?;
                Self::Format1 { coverage, delta }
            }
            2 => {
                let offset = s.read::<Offset16>()?.to_usize();
                let coverage = Coverage::parse(data.get(offset..)?)?;
                let count = s.read::<u16>()?;
                let substitutes = s.read_array16(count)?;
                Self::Format2 { coverage, substitutes }
            }
            _ => return None,
        })
    }

    fn coverage(&self) -> &Coverage<'a> {
        match self {
            Self::Format1 { coverage, .. } => coverage,
            Self::Format2 { coverage, .. } => coverage,
        }
    }

    fn would_apply(&self, ctx: &WouldApplyContext) -> bool {
        let glyph_id = GlyphId(u16::try_from(ctx.glyph(0)).unwrap());
        ctx.len() == 1 && self.coverage().get(glyph_id).is_some()
    }

    fn apply(&self, ctx: &mut ApplyContext) -> Option<()> {
        let glyph_id = GlyphId(u16::try_from(ctx.buffer().cur(0).codepoint).unwrap());
        let subst = match self {
            Self::Format1 { coverage, delta } => {
                coverage.get(glyph_id)?;
                // According to the Adobe Annotated OpenType Suite, result is always
                // limited to 16bit.
                GlyphId((i32::from(glyph_id.0) + i32::from(*delta)) as u16)
            }
            Self::Format2 { coverage, substitutes } => {
                let index = coverage.get(glyph_id)?;
                substitutes.get(index)?
            }
        };

        ctx.replace_glyph(subst);
        Some(())
    }
}

#[derive(Clone, Copy, Debug)]
enum MultipleSubst<'a> {
    Format1 {
        coverage: Coverage<'a>,
        sequences: Offsets16<'a, Offset16>,
    }
}

impl<'a> MultipleSubst<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let format = s.read::<u16>()?;
        Some(match format {
            1 => {
                let offset = s.read::<Offset16>()?.to_usize();
                let coverage = Coverage::parse(data.get(offset..)?)?;
                let count = s.read::<u16>()?;
                let sequences = s.read_offsets16(count, data)?;
                Self::Format1 { coverage, sequences }
            }
            _ => return None,
        })
    }

    fn coverage(&self) -> &Coverage<'a> {
        match self {
            Self::Format1 { coverage, .. } => coverage,
        }
    }

    fn would_apply(&self, ctx: &WouldApplyContext) -> bool {
        let glyph_id = GlyphId(u16::try_from(ctx.glyph(0)).unwrap());
        ctx.len() == 1 && self.coverage().get(glyph_id).is_some()
    }

    fn apply(&self, ctx: &mut ApplyContext) -> Option<()> {
        let glyph_id = GlyphId(u16::try_from(ctx.buffer().cur(0).codepoint).unwrap());
        match self {
            Self::Format1 { coverage, sequences } => {
                let index = coverage.get(glyph_id)?;
                let seq = Sequence::parse(sequences.slice(index)?)?;
                seq.apply(ctx)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Sequence<'a> {
    substitutes: LazyArray16<'a, GlyphId>,
}

impl<'a> Sequence<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let count = s.read::<u16>()?;
        let substitutes = s.read_array16(count)?;
        Some(Self { substitutes })
    }

    fn apply(&self, ctx: &mut ApplyContext) -> Option<()> {
        match self.substitutes.len() {
            // Spec disallows this, but Uniscribe allows it.
            // https://github.com/harfbuzz/harfbuzz/issues/253
            0 => ctx.buffer_mut().delete_glyph(),

            // Special-case to make it in-place and not consider this
            // as a "multiplied" substitution.
            1 => ctx.replace_glyph(self.substitutes.get(0)?),

            _ => {
                let class = if ctx.buffer().cur(0).is_ligature() {
                    GlyphPropsFlags::BASE_GLYPH
                } else {
                    GlyphPropsFlags::empty()
                };

                for (i, subst) in self.substitutes.into_iter().enumerate() {
                    ctx.buffer_mut().cur_mut(0).set_lig_props_for_component(i as u8);
                    ctx.output_glyph_for_component(subst, class);
                }

                ctx.buffer_mut().skip_glyph();
            }
        }
        Some(())
    }
}

#[derive(Clone, Copy, Debug)]
enum AlternateSubst<'a> {
    Format1 {
        coverage: Coverage<'a>,
        alternate_sets: Offsets16<'a, Offset16>,
    }
}

impl<'a> AlternateSubst<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let format = s.read::<u16>()?;
        Some(match format {
            1 => {
                let offset = s.read::<Offset16>()?.to_usize();
                let coverage = Coverage::parse(data.get(offset..)?)?;
                let count = s.read::<u16>()?;
                let alternate_sets = s.read_offsets16(count, data)?;
                Self::Format1 { coverage, alternate_sets }
            }
            _ => return None,
        })
    }

    fn coverage(&self) -> &Coverage<'a> {
        match self {
            Self::Format1 { coverage, .. } => coverage,
        }
    }

    fn would_apply(&self, ctx: &WouldApplyContext) -> bool {
        let glyph_id = GlyphId(u16::try_from(ctx.glyph(0)).unwrap());
        ctx.len() == 1 && self.coverage().get(glyph_id).is_some()
    }

    fn apply(&self, ctx: &mut ApplyContext) -> Option<()> {
        let glyph_id = GlyphId(u16::try_from(ctx.buffer().cur(0).codepoint).unwrap());
        match self {
            Self::Format1 { coverage, alternate_sets } => {
                let index = coverage.get(glyph_id)?;
                let set = AlternateSet::parse(alternate_sets.slice(index)?)?;
                set.apply(ctx)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct AlternateSet<'a> {
    alternates: LazyArray16<'a, GlyphId>,
}

impl<'a> AlternateSet<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let count = s.read::<u16>()?;
        let alternates = s.read_array16(count)?;
        Some(Self { alternates })
    }

    fn apply(&self, ctx: &mut ApplyContext) -> Option<()> {
        let len = self.alternates.len() as u32;
        if len == 0 {
            return None;
        }

        let glyph_mask = ctx.buffer().cur(0).mask;
        let lookup_mask = ctx.lookup_mask();

        // Note: This breaks badly if two features enabled this lookup together.
        let shift = lookup_mask.trailing_zeros();
        let mut alt_index = (lookup_mask & glyph_mask) >> shift;

        // If alt_index is MAX_VALUE, randomize feature if it is the rand feature.
        if alt_index == Map::MAX_VALUE && ctx.random() {
            alt_index = ctx.random_number() % len + 1;
        }

        let idx = u16::try_from(alt_index).ok()?.checked_sub(1)?;
        ctx.replace_glyph(self.alternates.get(idx)?);

        Some(())
    }
}

macro_rules! make_ffi_funcs {
    ($table:ident, $would_apply:ident, $apply:ident) => {
        #[no_mangle]
        pub extern "C" fn $would_apply(
            ctx: *const ffi::rb_would_apply_context_t,
            data_ptr: *const u8,
            data_len: u32,
        ) -> ffi::rb_bool_t {
            let ctx = WouldApplyContext::from_ptr(ctx);
            let data = unsafe { std::slice::from_raw_parts(data_ptr, data_len as usize) };
            match $table::parse(data) {
                Some(table) => table.would_apply(&ctx) as ffi::rb_bool_t,
                None => 0,
            }
        }

        #[no_mangle]
        pub extern "C" fn $apply(
            ctx: *mut ffi::rb_ot_apply_context_t,
            data_ptr: *const u8,
            data_len: u32,
        ) -> ffi::rb_bool_t {
            let mut ctx = ApplyContext::from_ptr_mut(ctx);
            let data = unsafe { std::slice::from_raw_parts(data_ptr, data_len as usize) };
            match $table::parse(data) {
                Some(table) => table.apply(&mut ctx).is_some() as ffi::rb_bool_t,
                None => 0,
            }
        }
    }
}

make_ffi_funcs!(SingleSubst, rb_single_subst_would_apply, rb_single_subst_apply);
make_ffi_funcs!(MultipleSubst, rb_multiple_subst_would_apply, rb_multiple_subst_apply);
make_ffi_funcs!(AlternateSubst, rb_alternate_subst_would_apply, rb_alternate_subst_apply);
