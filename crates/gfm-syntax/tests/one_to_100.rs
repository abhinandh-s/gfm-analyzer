// # ATX Headings


use gfm_syntax::cst;
use gfm_syntax::assert_tree;

assert_tree!(
    ath_heading,
    example_32, // https://github.github.com/gfm/#example-32
    "# foo
## foo
### foo
#### foo
##### foo
###### foo"
);

assert_tree!(
    ath_heading,
    example_33, // https://github.github.com/gfm/#example-33
    "####### foo"
);

assert_tree!(
    ath_heading,
    example_34, // https://github.github.com/gfm/#example-34
    "#5 bolt

#hashtag"
);

assert_tree!(
    ath_heading,
    example_35, // https://github.github.com/gfm/#example-35
    r#"\## foo"#
);

assert_tree!(
    ath_heading,
    example_36, // https://github.github.com/gfm/#example-36
    r#"# foo *bar* \*baz\*"#
);

assert_tree!(
    ath_heading,
    example_37, // https://github.github.com/gfm/#example-37
    "#                  foo                     "
);
assert_tree!(
    ath_heading,
    example_38, // https://github.github.com/gfm/#example-38
    r#" ### foo
  ## foo
   # foo"#
);
assert_tree!(
    ath_heading,
    example_39, // https://github.github.com/gfm/#example-39
    r#"    # foo"#
);

assert_tree!(
    ath_heading,
    example_40, // https://github.github.com/gfm/#example-40
    r#"foo
    # bar"#
);
assert_tree!(
    ath_heading,
    example_41, // https://github.github.com/gfm/#example-41
    r#"## foo ##
  ###   bar    ###"#
);
assert_tree!(
    ath_heading,
    example_42, // https://github.github.com/gfm/#example-42
    r#"# foo ##################################
##### foo ##"#
);
assert_tree!(
    ath_heading,
    example_43, // https://github.github.com/gfm/#example-43
    "### foo ###     "
);
assert_tree!(
    ath_heading,
    example_44, // https://github.github.com/gfm/#example-44
    "### foo ### b"
);
assert_tree!(
    ath_heading,
    example_45, // https://github.github.com/gfm/#example-45
    "# foo#"
);
assert_tree!(
    ath_heading,
    example_46, // https://github.github.com/gfm/#example-46
    r#"### foo \###
## foo #\##
# foo \#"#
);

// assert_tree!(
//     ath_heading,
//     example_47, // https://github.github.com/gfm/#example-47
//     "****
// ## foo
// ****"
// );
// assert_tree!(
//     ath_heading,
//     example_48, // https://github.github.com/gfm/#example-48
//     "Foo bar
// # baz
// Bar foo"
// );
// assert_tree!(
//     ath_heading,
//     example_49, // https://github.github.com/gfm/#example-49
//     "##
// #
// ### ###"
// );

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
    r#"    aaa
bbb"#
);
