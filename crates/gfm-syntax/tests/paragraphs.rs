//! Paragraph [Done]
//!
//! Example [189..196]

use gfm_syntax::assert_tree;
use gfm_syntax::cst;

assert_tree!(
    paragraph,
    example_189, // https://github.github.com/gfm/#example-189
    r#"aaa

bbb"#
);

assert_tree!(
    paragraph,
    example_190, // https://github.github.com/gfm/#example-190
    r#"aaa
bbb

ccc
ddd"#
);

assert_tree!(
    paragraph,
    example_191, // https://github.github.com/gfm/#example-191
    r#"aaa


bbb"#
);

assert_tree!(
    paragraph,
    example_192, // https://github.github.com/gfm/#example-192
    r#"  aaa
 bbb"#
);

assert_tree!(
    paragraph,
    example_193, // https://github.github.com/gfm/#example-193
    r#"aaa
             bbb
                                       ccc"#
);

assert_tree!(
    paragraph,
    example_194, // https://github.github.com/gfm/#example-194
    r#"   aaa
bbb"#
);

assert_tree!(
    paragraph,
    example_195, // https://github.github.com/gfm/#example-195
    r#"    aaa
bbb"#
);

assert_tree!(
    paragraph,
    example_196, // https://github.github.com/gfm/#example-196
    r#"aaa     
bbb     "#
);
