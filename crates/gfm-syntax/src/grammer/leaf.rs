use crate::*;
use grammer::*;

pub const EXAMPLE_0: &str = r#"foo"#;

pub(super) fn indented_code_blocks(p: &mut Parser, stop_at_newline: bool) -> usize {
    let m = p.start();
    let count = match stop_at_newline {
        true => indented_code_block(p),
        false => {
            let mut n = 0;
            while !p.is_at_eof() {
                n = indented_code_block(p);
                if n >= 4 {
                    n = 0;
                    continue;
                } else {
                    break;
                }
            }
            n
        }
    };
    p.wrap(m, SyntaxKind::IndentedCodeBlock);
    count
}

// # Indented code blocks
//
// An indented code block is composed of one or more indented chunks separated by blank lines.
// An indented chunk is a sequence of non-blank lines, each indented four or more spaces.
// The contents of the code block are the literal contents of the lines, including trailing line endings,
// minus four spaces of indentation. An indented code block has no info string.
// An indented code block cannot interrupt a paragraph, so there must be a blank line between
// a paragraph and a following indented code block. (A blank line is not needed,
// however, between a code block and a following paragraph.)
//
//
// returns WhiteSpace count of new line
fn indented_code_block(p: &mut Parser) -> usize {
    p.eat_until(syntax_set!(LineEnding, ParaBreak, Eof));
    match p.current() {
        T![LineEnding] | T![ParaBreak] => eat_breaks(p),
        _ => 0,
    }
}

// Example [77..88]

assert_tree!(indented_code_blocks, example_77, EXAMPLE_77);
// assert_tree!(indented_code_blocks, example_78, EXAMPLE_78);
// assert_tree!(indented_code_blocks, example_79, EXAMPLE_79);
// assert_tree!(indented_code_blocks, example_80, EXAMPLE_80);
assert_tree!(indented_code_blocks, example_81, EXAMPLE_81);
assert_tree!(indented_code_blocks, example_82, EXAMPLE_82);
assert_tree!(indented_code_blocks, example_83, EXAMPLE_83);
assert_tree!(indented_code_blocks, example_84, EXAMPLE_84);
// assert_tree!(indented_code_blocks, example_85, EXAMPLE_85);
assert_tree!(indented_code_blocks, example_86, EXAMPLE_86);
assert_tree!(indented_code_blocks, example_87, EXAMPLE_87);
assert_tree!(indented_code_blocks, example_88, EXAMPLE_88);
