use lexer::Lexer;

use crate::{lexer::cursor::Cursor, common::source::Source};

pub mod lexer;
pub mod common;

fn main() {
    let source = Source {
        code: r#"
            function sum(a: int, b: int) -> int {
                return a + b;
            }
            function main() {
                print(sum(1, 2));
            }
        "#,
        path: "main.lang"
    };
    let lexer = Lexer::new(source);
    for token in lexer {
        println!("{:?}", token);
    }
}
