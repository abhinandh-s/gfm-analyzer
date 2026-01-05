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

    let mut count = p.skip_whitespace();
    // stops on Eof
    p.iter_while(None, |p| {
        // let count = p.skip_whitespace();

        match p.current() {
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
                count = paragraph(p);
            }
        }
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
    let mut count = 0;
    let m = p.start();
    let last_cursor = p.cursor;
    looper!(!p.is_at_eof(), {
        match p.current() {
            SyntaxKind::LineEnding => {
                count = eat_breaks(p);
                // break;
            }
            SyntaxKind::ParaBreak => {
                count = eat_breaks(p);
                break;
            }
            _ => paragraph_segment(p),
        }
    });

    p.assert_movement(last_cursor);

    p.wrap(m, SyntaxKind::Paragraph);
    count
}

fn quote(p: &mut Parser) {
    let m = p.start();
    p.eat_many_in_set(syntax_set!(GreaterThan));
    p.expect(T![WhiteSpace]);
    paragraph_segment(p);
    p.wrap(m, T![Quote]);
}

fn code(p: &mut Parser) {
    let m = p.start();
    p.eat_until(syntax_set!(LineEnding, ParaBreak));
    p.wrap(m, T![InlineCode]);
}

// Example: 32, 33, 34, 35, -36- , 37, 38, 39, 40, 41, -42-, -43-, -44-
fn heading(p: &mut Parser) {
    p.assert(T![Pound]);
    let m = p.start();
    let count = p.eat_many_counted(T![Pound]);
    let mut end_count = 0;
    let exists = p.expect(T![WhiteSpace]);
    let terminators = syntax_set!(LineEnding, ParaBreak, Eof);

    while !p.at_set(terminators) {
        // # foo #####
        //      ^
        //      stops here
        if (p.current() == T![WhiteSpace]) && (p.next() == Some(T![Pound])) {
            break;
        }
        p.eat();
    }

    // # foo #####
    //      ^
    //      parse from here
    if (p.current() == T![WhiteSpace]) && (p.next() == Some(T![Pound])) {
    p.eat();
    // only `#` left
    }

    // # foo ##### dhosfnlfdjkhjk
    //            ^
    //            from here is dealed

    // if (p.current() == T![WhiteSpace]) && (p.next() == Some(T![Pound])) {
    //     p.eat(); // WhiteSpace
    //     println!("this is Pound: => {}", p.current());
    //     // end_count = p.eat_many_counted(T![Pound]);
    //     let mut n = 0;
    //     while p.at(T![Pound]) && p.next() == Some(SyntaxKind::Pound) {
    //         p.eat();
    //         n += 1;
    //     }
    //     // # foo #####
    //     //          ^ one more `#` to eat
    //     n += 1;
    //     end_count = n;
    // }
    // println!("this is : => {}", p.current());
    // //
    //
    //        // p.assert(T![Pound]);
    //       //  p.expect(T![Pound]);
    //         end_count = n;
    //
    //         // # foo ######
    //         //            ^ we are at last `#`
    //
    //
    //
    //
    //         if count > end_count {
    //
    //             p.expect(T![Pound]);
    //         }
    //
    //         // if p.current() == SyntaxKind::Pound && p.next() != Some(SyntaxKind::Pound) {
    //        // if count > end_count {
    //         //    p.expect(T![Pound]);
    //         //     //                 end_count += 1;
    //      //   } else if count < end_count {
    //        //     p.unexpected_with_hint("consider removing this extra `#`");
    //       //  }
    //         //
    //         //     // current is `#` and next is also `#`
    //         // } else {
    //         // end_count += 1;
    //         // }
    //        // p.eat_until(terminators);
    //     } else {
    //         p.eat();
    //     }
    // }
    // 01) havn't considered the optional `#` sequence at the end rule

    println!("count: {}, end_count: {}", count, end_count);
    if !p.at_set(terminators) {
        p.eat_until(terminators);
    }
    if count <= 6 && exists {
        p.wrap(m, SyntaxKind::Heading);
    } else {
        p.wrap(m, SyntaxKind::Paragraph);
    }
}

fn linkable(p: &mut Parser) {
    // {./README.md}[readme]
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
