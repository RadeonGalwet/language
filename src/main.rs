use lexer::Lexer;
use parser::{cursor::Cursor, Parser};

use crate::{common::source::Source, lexer::cursor::slice::utf8_slice};

pub mod common;
pub mod lexer;
pub mod parser;

fn main() {
    let source = Source {
        code: r#"
            function main(args: Arguments) {
                let mut a = 0;
                while(a < 10) {
                    if(a == 5) {
                        print(a);
                    }
                    print(sum(2, 3));
                    a = a + 1;
                }
            }
            function sum(a: int, b: int) -> int {
                return a + b;
            }
        "#,
        path: "main.lang",
    };
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(source, Cursor::new(lexer.peekable(), source));
    match parser.parse_program() {
        Ok(ast) => println!("{:#?}", ast),
        Err(err) => {
            let err = *err;
            println!(
                "{:?} at {}.{}\n{}",
                err.kind,
                err.span.start,
                err.span.end,
                utf8_slice(source.code, err.span.start - 5, err.span.end + 5)
            )
        }
    }
}
