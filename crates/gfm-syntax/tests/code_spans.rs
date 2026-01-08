//! Code Spans
//!
//! Examples [338..359]

use gfm_syntax::assert_tree;
use gfm_syntax::cst;

assert_tree!(
    blank_lines,
    example_338,
    r###"`foo`"###
);

assert_tree!(
    blank_lines,
    example_339,
    r###"`` foo ` bar ``"###
);

// assert_tree!(
//     blank_lines,
//     example_340,
//     r###"`foo`"###
// );
//
// assert_tree!(
//     blank_lines,
//     example_341,
//     r###"`foo`"###
// );
// assert_tree!(
//     blank_lines,
//     example_342,
//     r###"`foo`"###
// );
// assert_tree!(
//     blank_lines,
//     example_343,
//     r###"`foo`"###
// );
// assert_tree!(
//     blank_lines,
//     example_344,
//     r###"`foo`"###
// );
// assert_tree!(
//     blank_lines,
//     example_345,
//     r###"`foo`"###
// );
// assert_tree!(
//     blank_lines,
//     example_346,
//     r###"`foo`"###
// );
//
// assert_tree!(
//     blank_lines,
//     example_347,
//     r###"`foo`"###
// );
//
// assert_tree!(
//     blank_lines,
//     example_348,
//     r###"`foo`"###
// );
//
// assert_tree!(
//     blank_lines,
//     example_349,
//     r###"`foo`"###
// );
//
// assert_tree!(
//     blank_lines,
//     example_350,
//     r###"`foo`"###
// );
//
// assert_tree!(
//     blank_lines,
//     example_351,
//     r###"`foo`"###
// );
//
// assert_tree!(
//     blank_lines,
//     example_352,
//     r###"`foo`"###
// );
//
// assert_tree!(
//     blank_lines,
//     example_353,
//     r###"`foo`"###
// );
//
// assert_tree!(
//     blank_lines,
//     example_354,
//     r###"`foo`"###
// );
//
// assert_tree!(
//     blank_lines,
//     example_355,
//     r###"`foo`"###
// );
//
// assert_tree!(
//     blank_lines,
//     example_356,
//     r###"`foo`"###
// );
//
// assert_tree!(
//     blank_lines,
//     example_357,
//     r###"`foo`"###
// );
//
// assert_tree!(
//     blank_lines,
//     example_358,
//     r###"`foo`"###
// );
//
// assert_tree!(
//     blank_lines,
//     example_359,
//     r###"`foo`"###
// );

