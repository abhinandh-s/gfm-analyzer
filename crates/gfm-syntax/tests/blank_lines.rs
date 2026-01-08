//! Only one example

use gfm_syntax::assert_tree;
use gfm_syntax::cst;

assert_tree!(
    blank_lines,
    example_196,
    r###"
  

aaa
  

# aaa

   
"###
);
