use gfm_syntax::cst;
use gfm_syntax::assert_tree;

assert_tree!(
    hard_line_breaks,
    example_658, // https://github.github.com/gfm/#example-658
    r###"foo  
baz"###
);
