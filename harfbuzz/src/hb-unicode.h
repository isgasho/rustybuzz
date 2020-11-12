/*
 * Copyright © 2009  Red Hat, Inc.
 * Copyright © 2011  Codethink Limited
 * Copyright © 2011,2012  Google, Inc.
 *
 *  This is part of HarfBuzz, a text shaping library.
 *
 * Permission is hereby granted, without written agreement and without
 * license or royalty fees, to use, copy, modify, and distribute this
 * software and its documentation for any purpose, provided that the
 * above copyright notice and the following two paragraphs appear in
 * all copies of this software.
 *
 * IN NO EVENT SHALL THE COPYRIGHT HOLDER BE LIABLE TO ANY PARTY FOR
 * DIRECT, INDIRECT, SPECIAL, INCIDENTAL, OR CONSEQUENTIAL DAMAGES
 * ARISING OUT OF THE USE OF THIS SOFTWARE AND ITS DOCUMENTATION, EVEN
 * IF THE COPYRIGHT HOLDER HAS BEEN ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 *
 * THE COPYRIGHT HOLDER SPECIFICALLY DISCLAIMS ANY WARRANTIES, INCLUDING,
 * BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND
 * FITNESS FOR A PARTICULAR PURPOSE.  THE SOFTWARE PROVIDED HEREUNDER IS
 * ON AN "AS IS" BASIS, AND THE COPYRIGHT HOLDER HAS NO OBLIGATION TO
 * PROVIDE MAINTENANCE, SUPPORT, UPDATES, ENHANCEMENTS, OR MODIFICATIONS.
 *
 * Red Hat Author(s): Behdad Esfahbod
 * Codethink Author(s): Ryan Lortie
 * Google Author(s): Behdad Esfahbod
 */

#ifndef RB_H_IN
#error "Include <hb.h> instead."
#endif

#ifndef RB_UNICODE_H
#define RB_UNICODE_H

#include "hb-common.h"

RB_BEGIN_DECLS

/* rb_unicode_general_category_t */

/* Unicode Character Database property: General_Category (gc) */
typedef enum {
    RB_UNICODE_GENERAL_CATEGORY_CONTROL,             /* Cc */
    RB_UNICODE_GENERAL_CATEGORY_FORMAT,              /* Cf */
    RB_UNICODE_GENERAL_CATEGORY_UNASSIGNED,          /* Cn */
    RB_UNICODE_GENERAL_CATEGORY_PRIVATE_USE,         /* Co */
    RB_UNICODE_GENERAL_CATEGORY_SURROGATE,           /* Cs */
    RB_UNICODE_GENERAL_CATEGORY_LOWERCASE_LETTER,    /* Ll */
    RB_UNICODE_GENERAL_CATEGORY_MODIFIER_LETTER,     /* Lm */
    RB_UNICODE_GENERAL_CATEGORY_OTHER_LETTER,        /* Lo */
    RB_UNICODE_GENERAL_CATEGORY_TITLECASE_LETTER,    /* Lt */
    RB_UNICODE_GENERAL_CATEGORY_UPPERCASE_LETTER,    /* Lu */
    RB_UNICODE_GENERAL_CATEGORY_SPACING_MARK,        /* Mc */
    RB_UNICODE_GENERAL_CATEGORY_ENCLOSING_MARK,      /* Me */
    RB_UNICODE_GENERAL_CATEGORY_NON_SPACING_MARK,    /* Mn */
    RB_UNICODE_GENERAL_CATEGORY_DECIMAL_NUMBER,      /* Nd */
    RB_UNICODE_GENERAL_CATEGORY_LETTER_NUMBER,       /* Nl */
    RB_UNICODE_GENERAL_CATEGORY_OTHER_NUMBER,        /* No */
    RB_UNICODE_GENERAL_CATEGORY_CONNECT_PUNCTUATION, /* Pc */
    RB_UNICODE_GENERAL_CATEGORY_DASH_PUNCTUATION,    /* Pd */
    RB_UNICODE_GENERAL_CATEGORY_CLOSE_PUNCTUATION,   /* Pe */
    RB_UNICODE_GENERAL_CATEGORY_FINAL_PUNCTUATION,   /* Pf */
    RB_UNICODE_GENERAL_CATEGORY_INITIAL_PUNCTUATION, /* Pi */
    RB_UNICODE_GENERAL_CATEGORY_OTHER_PUNCTUATION,   /* Po */
    RB_UNICODE_GENERAL_CATEGORY_OPEN_PUNCTUATION,    /* Ps */
    RB_UNICODE_GENERAL_CATEGORY_CURRENCY_SYMBOL,     /* Sc */
    RB_UNICODE_GENERAL_CATEGORY_MODIFIER_SYMBOL,     /* Sk */
    RB_UNICODE_GENERAL_CATEGORY_MATH_SYMBOL,         /* Sm */
    RB_UNICODE_GENERAL_CATEGORY_OTHER_SYMBOL,        /* So */
    RB_UNICODE_GENERAL_CATEGORY_LINE_SEPARATOR,      /* Zl */
    RB_UNICODE_GENERAL_CATEGORY_PARAGRAPH_SEPARATOR, /* Zp */
    RB_UNICODE_GENERAL_CATEGORY_SPACE_SEPARATOR      /* Zs */
} rb_unicode_general_category_t;

RB_EXTERN rb_codepoint_t rb_ucd_mirroring(rb_codepoint_t cp);

RB_END_DECLS

#endif /* RB_UNICODE_H */
