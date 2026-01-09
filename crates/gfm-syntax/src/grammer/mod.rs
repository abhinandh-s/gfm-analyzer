#![allow(dead_code)]

use crate::*;

mod am;
use am::*;

mod dm;
use dm::*;

pub enum NeorgLint {
    EmptyAtModifiers,
    EmptyLink,
}

// document
//
// - paragraph_break and line_break (x) // lexer will handle both line_break & paragraph_break
// - paragraph (x)
// - heading (x)
//       | nestable_detached_modifier
//             | rangeable_detached_modifier
//             | table
//             | tag
//             | horizontal_line (X)
//             | strong_paragraph_delimiter
//
pub fn document(p: &mut Parser) -> SyntaxNode {
    let m = p.start();

    let mut n = 0;
    let mut count = p.skip_whitespace();
    // stops on Eof
    p.iter_while(None, |p| {
        // let count = p.skip_whitespace();

        println!("n = {n}, current: {}", p.current());

        match p.current() {
            T![WhiteSpace] => {
                count = p.skip_whitespace();
            }
            T![Pound] => {
                if count < 3 {
                    heading(p)
                } else {
                    code(p);
                }
            }
            T![GreaterThan] => quote(p),
            T![Hyphen] => unorderedlist(p),
            _ => {
                if count > 3 {
                    code(p);
                    count = 0;
                } else {
                    count = paragraph(p)
                }
            }
        }
        n += 1;
    });
    p.bump(T![Eof]);
    p.wrap(m, T![Document]);
    std::mem::take(&mut p.nodes[0])
}

// // A paragraph segment can contain any paragraph element.
// paragraph_segment: $ =>
// prec.right(0,
//   seq(
//     optional($.weak_carryover_set),
//     repeat1(
//       choice(
//         $._paragraph_element,
//         alias($._conflict_open, "_word"),
//       ),
//     ),
//   ),
// ),
fn paragraph_segment(p: &mut Parser) {
    let m = p.start();

    while !p.is_at_eof() {
        match p.current() {
            SyntaxKind::Eof | SyntaxKind::LineEnding => {
                p.bump_line();
                p.eat();
                break;
            }
            SyntaxKind::LessThan => {
                let m = p.start();
                p.eat_until(syntax_set!(LineEnding, ParaBreak, GreaterThan));
                p.wrap(m, SyntaxKind::HtmlTag);
            }
            SyntaxKind::ParaBreak => break,
            SyntaxKind::LCurly => linkable(p),
            any if DELIMITER_PAIR.contains(&any) => parse_attached_modifiers(p),
            _ => p.eat(),
        }
    }
    p.wrap(m, T![ParaSegment]);
}

// paragraph_element => word
//                      | space
//                      | trailing_modifier
//                      | link
//                      | anchor_declaration
//                      | anchor_definition
//                      | inline_link_target
//                      | escape_sequence
//                      | link_modifier?
//                      | attached_modifier?
//
fn paragraph_element(p: &mut Parser) {
    let c = p.current();
    match c {
        T![Word] | T![WhiteSpace] | T![Tab] => p.eat(),
        SyntaxKind::RParen => (),
        SyntaxKind::RCurly => (),
        SyntaxKind::RSquare => (),
        SyntaxKind::LessThan => {
            let m = p.start();
            p.eat_until(syntax_set!(LineEnding, ParaBreak, LessThan));
            p.wrap(m, SyntaxKind::HtmlTag);
        }
        any if ATTACHED_MODIFIERS.contains(any) => parse_attached_modifiers(p),
        _ => (),
    }
}

fn eat_breaks(p: &mut Parser) -> usize {
    let mut count = 0;
    while !p.is_at_eof() {
        match p.current() {
            SyntaxKind::LineEnding => {
                p.eat_and_get().text().chars().for_each(|ch| match ch {
                    '\n' => count = 0,
                    '\t' => {
                        count += 4;
                    }
                    _ => {
                        count += 1;
                    }
                });
                p.bump_line();
            }
            SyntaxKind::ParaBreak => {
                p.eat_and_get().text().chars().for_each(|ch| match ch {
                    '\n' => count = 0,
                    '\t' => {
                        count += 4;
                    }
                    _ => {
                        count += 1;
                    }
                });
                p.bump_line();
                p.bump_line();
            }
            _ => break,
        }
    }
    count
}

fn paragraph(p: &mut Parser) -> usize {
    let m = p.start();
    let count = paragraph_unwarped(p);
    p.wrap(m, SyntaxKind::Paragraph);
    count
}

fn paragraph_unwarped(p: &mut Parser) -> usize {
    let mut count = 0;
    let last_cursor = p.cursor;
    looper!(!p.is_at_eof(), {
        match p.current() {
            SyntaxKind::LineEnding => {
                count = eat_breaks(p);
                continue;
            }
            SyntaxKind::ParaBreak => {
                eat_breaks(p);
                break;
            }
            SyntaxKind::Pound => {
                if count < 3 {
                    break;
                } else {
                    p.eat();
                }
            }
            SyntaxKind::GreaterThan => {
                if count < 3 {
                    break;
                } else {
                    p.eat();
                }
            }
            SyntaxKind::Backtick => code(p),
            _ => {
                if count < 3 {
                    inline(p);
                } else {
                    code(p);
                }
            }
        }
        count = 0;
    });

    p.assert_movement(last_cursor);
    count
}

fn quote(p: &mut Parser) {
    let m = p.start();
    while !p.is_at_eof() && p.current() == T![GreaterThan] {
        p.eat_many_in_set(syntax_set!(GreaterThan));
        // this space can be ommited, so [skip_whitespace]
        let mut count = p.skip_whitespace();
        match p.current() {
            SyntaxKind::Pound => heading(p),
            _ => {
                if count < 3 {
                    let m = p.start();
                    inline(p);
                    if p.current() == T![LineEnding] && p.next() == Some(T![GreaterThan]) {
                        count = eat_breaks(p);
                    }
                    p.wrap(m, T![Paragraph]);
                } else {
                    code(p);
                }
            }
        }
    }
    p.wrap(m, T![Quote]);
}

fn code(p: &mut Parser) {
    let m = p.start();
    println!("in code: c = {}", p.current());
    let end_count: usize = 0;
    let count = match p.current() {
        T![Backtick] => p.eat_many_counted(T![Backtick]),
        _ => 0,
    };
    if count == 0 {
        p.eat_until(syntax_set!(LineEnding, ParaBreak));
    }
    while !p.is_at_eof() {
        println!("in code: c = {}", p.current());
        p.eat_until(syntax_set!(Backtick));
        let end_count = p.eat_many_counted(T![Backtick]);
        if count == end_count {
            break;
        }
    }
    if !p.is_at_eof() {
        if count > end_count {
            p.expect(T![Backtick]);
        } else if count < end_count {
            p.expect_closing_delimiter(m, T![Backtick]);
        }
        eat_breaks(p);
    }
    println!("in code: c = {}", p.current());

    p.wrap(m, T![InlineCode]);
}

#[allow(clippy::print_stdout)]
fn inline(p: &mut Parser) {
    let m = p.start();

    let mut n = 0;

    while !p.is_at_eof() {
        println!("{}", n);
        println!("{}", p.current());
        match p.current() {
            SyntaxKind::ForwardSlash if p.next() == Some(T![LineEnding]) => {
                let m = p.start();
                p.eat();
                p.wrap(m, SyntaxKind::HardLineBreaks);
            }
            SyntaxKind::WhiteSpace if p.next() == Some(T![LineEnding]) => {
                let m = p.start();
                let n = p.eat_and_get().text().chars().count();
                if n >= 2 {
                    p.wrap(m, SyntaxKind::HardLineBreaks);
                }
            }
            SyntaxKind::LessThan => {
                let m = p.start();
                p.eat_until(syntax_set!(LineEnding, ParaBreak, GreaterThan));
                p.wrap(m, SyntaxKind::HtmlTag);
            }
            SyntaxKind::ParaBreak | SyntaxKind::LineEnding => break,
            SyntaxKind::LCurly => linkable(p),
            any if DELIMITER_PAIR.contains(&any) => parse_attached_modifiers(p),
            any if (p.current() == T![WhiteSpace]) && (p.next() == Some(T![Pound])) => break,
            _ => {
                p.eat();
            }
        }
        n += 1;
    }
    p.wrap(m, T![Inline]);
}

#[allow(clippy::print_stdout)]
fn heading(p: &mut Parser) {
    p.assert(T![Pound]);
    let m = p.start();
    let count = p.eat_many_counted(T![Pound]);
    let is_termination = matches!(p.current(), T![LineEnding] | T![ParaBreak]);
    let exist = match is_termination {
        true => false,
        false => p.expect(T![WhiteSpace]),
    };
    // let mut got_inline = false;
    let terminators = syntax_set!(LineEnding, ParaBreak, Eof);
    while !p.at_set(terminators) {
        // if p.current() == T![Word] { got_inline = true }
        if (p.current() == T![WhiteSpace]) && (p.next() == Some(T![Pound])) {
            p.eat();
            p.eat_many(T![Pound]);
        } else if p.current() == T![Pound] && p.prev() == Some(T![WhiteSpace]) {
            p.eat_many(T![Pound]);
        } else {
            inline(p);
        }
    }
    eat_breaks(p);
    if count <= 6 && (exist || is_termination) {
        p.wrap(m, SyntaxKind::Heading);
    } else {
        p.wrap(m, SyntaxKind::Paragraph);
    }
}

fn thematic_breaks(p: &mut Parser) {
    let m = p.start();
    let kind = p.current();
    let current = matches!(
        kind,
        SyntaxKind::Asterisk | SyntaxKind::Hyphen | SyntaxKind::Underscore
    );
    let second = p.next().filter(|k| *k == kind).is_some();
    let third = p.nth(2).filter(|k| *k == kind).is_some();
    if current && second && third {
        p.eat_many(kind);
        p.skip_whitespace();
        p.wrap(m, SyntaxKind::ThematicBreaks);
    }
}

fn linkable(p: &mut Parser) {
    let m = p.start();

    let current = p.current();
    p.eat(); // `{`
    p.eat_until(syntax_set!(Eof, LineEnding, ParaBreak, RCurly));
    p.expect_closing_delimiter(m, current.corresponding_pair_unchecked());
    let current = p.current();
    p.expect(T![LSquare]);
    p.eat_until(syntax_set!(Eof, LineEnding, ParaBreak, RSquare));
    p.expect_closing_delimiter(m, current.corresponding_pair_unchecked());
    p.wrap(m, SyntaxKind::Link);
}
