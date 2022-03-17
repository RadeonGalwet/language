use lexer::Lexer;

use crate::{common::source::Source};

pub mod lexer;
pub mod common;

fn main() {
    let source = Source {
        code: r#"
            >= == == =
        "#,
        path: "main.lang"
    };
    let lexer = Lexer::new(source);
    for token in lexer {
        println!("{:?}", token);
    }
    
}
