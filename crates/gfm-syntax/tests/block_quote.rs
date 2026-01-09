//! Block Quote
//!
//!Example [206..230]

use gfm_syntax::assert_tree;
use gfm_syntax::cst;

assert_tree!(
    block_quotes,
    example_206,
    r###"> # Foo
> bar
> baz"###
);

assert_tree!(
    block_quotes,
    example_207,
    r###"># Foo
>bar
> baz"###
);

assert_tree!(
    block_quotes,
    example_208,
    r###"   > # Foo
   > bar
 > baz"###
);

assert_tree!(
    block_quotes,
    example_214,
    r###">     foo
    bar"###
);

//
// assert_tree!(
//     block_quotes,
//     example_206,
//     r###"foo"###
// );
//
// assert_tree!(
//     block_quotes,
//     example_206,
//     r###"foo"###
// );
//
// assert_tree!(
//     block_quotes,
//     example_206,
//     r###"foo"###
// );
//
// assert_tree!(
//     block_quotes,
//     example_206,
//     r###"foo"###
// );
//
// assert_tree!(
//     block_quotes,
//     example_206,
//     r###"foo"###
// );
//
// assert_tree!(
//     block_quotes,
//     example_206,
//     r###"foo"###
// );
//
// assert_tree!(
//     block_quotes,
//     example_206,
//     r###"foo"###
// );
//
// assert_tree!(
//     block_quotes,
//     example_206,
//     r###"foo"###
// );
//
//
// assert_tree!(
//     block_quotes,
//     example_206,
//     r###"foo"###
// );
//
// assert_tree!(
//     block_quotes,
//     example_206,
//     r###"foo"###
// );
//
// assert_tree!(
//     block_quotes,
//     example_206,
//     r###"foo"###
// );
//
// assert_tree!(
//     block_quotes,
//     example_206,
//     r###"foo"###
// );
//
// assert_tree!(
//     block_quotes,
//     example_206,
//     r###"foo"###
// );
//
// assert_tree!(
//     block_quotes,
//     example_206,
//     r###"foo"###
// );
//
// assert_tree!(
//     block_quotes,
//     example_206,
//     r###"foo"###
// );
//
// assert_tree!(
//     block_quotes,
//     example_206,
//     r###"foo"###
// );
//
//
// assert_tree!(
//     block_quotes,
//     example_206,
//     r###"foo"###
// );
//
// assert_tree!(
//     block_quotes,
//     example_206,
//     r###"foo"###
// );
//
// assert_tree!(
//     block_quotes,
//     example_206,
//     r###"foo"###
// );
//
// assert_tree!(
//     block_quotes,
//     example_206,
//     r###"foo"###
// );
//

