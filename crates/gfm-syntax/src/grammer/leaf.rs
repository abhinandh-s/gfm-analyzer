use crate::*;
use grammer::*;

fn op_delimiter(m: Marker, p: &mut Parser, kind: SyntaxKind) -> usize {
    let start_count = p.eat_many_counted(kind);
    p.skip_whitespace();
    p.wrap(m, SyntaxKind::Delimiter);
    start_count
}
fn cl_delimiter(p: &mut Parser, kind: SyntaxKind, start_count: usize) -> bool {
    let m = p.start();
    let end_count = p.eat_many_counted(kind);
    p.skip_whitespace();
    if !matches!(p.current(), T![LineEnding] | T![ParaBreak] | T![Eof]) {
        return false;
    }
    if start_count <= end_count {
        p.wrap(m, SyntaxKind::Delimiter);
        true
    } else {
        p.expect(kind);
        false
    }
}
// ## CST
//
//    - FencedCodeBlock
//      > Delimiter
//      > may contain InfoString
//      > Contents
//      > Delimiter
//
// ## Rules
//
// - 3 or more `Backtick` or `Tilda` (cannot be mixxed)
// - may contain `info_string` (this is trimmed of leading and trailing whitespace)
//   the info string comes after a backtick fence, it may not contain any backtick characters
// - until a closing code fence (with at least as many backticks or tildes as the opening code fence)
// -  If the leading code fence is indented N spaces,
//    then up to N spaces of indentation are removed from each line of the content (if present).
//    (If a content line is not indented, it is preserved unchanged.
//    If it is indented less than N spaces, all of the indentation is removed.)
//
pub(super) fn fenced_code_blocks(p: &mut Parser) -> usize {
    assert!(matches!(p.current(), T![Backtick] | T![Tilda]));
    let mut count = 0;
    let m = p.start();

    let kind = p.current();
    let terminators = syntax_set!(LineEnding, ParaBreak, Eof).add(kind);
    let start_count = op_delimiter(m, p, kind);
    // let mut _end_count = 0;

    let code_kind = if start_count >= 3 {
        T![FencedCodeBlock]
    } else {
        T![InlineCode]
    };
    if !terminators.contains(p.current()) {
        let w = p.start();
        p.eat();
        p.wrap(w, SyntaxKind::InfoString);
        count = indented_code_block(p);
    }

    while !p.is_at_eof() {
        p.eat_until(terminators);
        match p.current() {
            T![LineEnding] | T![ParaBreak] => {
                count = eat_breaks(p);
                if count >= 4 {
                    count = indented_code_block(p);
                } else {
                    continue;
                }
            }
            T![Eof] => break,
            any if any == kind => {
                if cl_delimiter(p, kind, start_count) {
                    break;
                }
            }
            _ => (),
        }

      
    }
    p.wrap(m, code_kind);
    count
}

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
/// returns WhiteSpace count of new line
fn indented_code_block(p: &mut Parser) -> usize {
    p.eat_until(syntax_set!(LineEnding, ParaBreak, Eof));
    match p.current() {
        T![LineEnding] | T![ParaBreak] => eat_breaks(p),
        _ => 0,
    }
}
// # Indented Code Blocks
//
// Example [77..88]
//
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

// # Fenced Code Blocks
//
// Example [89..117]
// assert_tree!(fenced_code_blocks, example_89, EXAMPLE_89);
// assert_tree!(fenced_code_blocks, example_90, EXAMPLE_90);
// assert_tree!(fenced_code_blocks, example_91, EXAMPLE_91);
// assert_tree!(fenced_code_blocks, example_92, EXAMPLE_92);
// assert_tree!(fenced_code_blocks, example_93, EXAMPLE_93);
// assert_tree!(fenced_code_blocks, example_94, EXAMPLE_94);
// assert_tree!(fenced_code_blocks, example_95, EXAMPLE_95);
// assert_tree!(fenced_code_blocks, example_96, EXAMPLE_96);
// assert_tree!(fenced_code_blocks, example_97, EXAMPLE_97);
// assert_tree!(fenced_code_blocks, example_98, EXAMPLE_98);
// assert_tree!(fenced_code_blocks, example_99, EXAMPLE_99);
// assert_tree!(fenced_code_blocks, example_100, EXAMPLE_100);
// assert_tree!(fenced_code_blocks, example_101, EXAMPLE_101);
// assert_tree!(fenced_code_blocks, example_102, EXAMPLE_102);
// assert_tree!(fenced_code_blocks, example_103, EXAMPLE_103);
// assert_tree!(fenced_code_blocks, example_104, EXAMPLE_104);
// assert_tree!(fenced_code_blocks, example_105, EXAMPLE_105);
// assert_tree!(fenced_code_blocks, example_106, EXAMPLE_106);
// assert_tree!(fenced_code_blocks, example_107, EXAMPLE_107);
// assert_tree!(fenced_code_blocks, example_108, EXAMPLE_108);
// assert_tree!(fenced_code_blocks, example_109, EXAMPLE_109);
// assert_tree!(fenced_code_blocks, example_110, EXAMPLE_110);
// assert_tree!(fenced_code_blocks, example_111, EXAMPLE_111);
// assert_tree!(fenced_code_blocks, example_112, EXAMPLE_112);
// assert_tree!(fenced_code_blocks, example_113, EXAMPLE_113);
// assert_tree!(fenced_code_blocks, example_114, EXAMPLE_114);
// assert_tree!(fenced_code_blocks, example_115, EXAMPLE_115);
// assert_tree!(fenced_code_blocks, example_116, EXAMPLE_116);
assert_tree!(fenced_code_blocks, example_117, EXAMPLE_117);

// # Paragraphs
//
// Example [189..196]
assert_tree!(paragraphs, example_189, EXAMPLE_189);
assert_tree!(paragraphs, example_190, EXAMPLE_190);
assert_tree!(paragraphs, example_191, EXAMPLE_191);
assert_tree!(paragraphs, example_192, EXAMPLE_192);
assert_tree!(paragraphs, example_193, EXAMPLE_193);
assert_tree!(paragraphs, example_194, EXAMPLE_194);
assert_tree!(paragraphs, example_195, EXAMPLE_195);
assert_tree!(paragraphs, example_196, EXAMPLE_196);

// # Blank lines
//
// Example 197
assert_tree!(blanklines, example_197, EXAMPLE_197);
