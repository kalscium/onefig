use std::fs;
use flexar::lext::Lext;
use onefig::{lexer::Token, nodes::source_file::SourceFile};

fn main() {
    let file = fs::read_to_string("example.of").unwrap();
    let tokens = Token::tokenize(Lext::new("example.of".into(), &file));
    println!("{:?}", tokens.iter()
        .map(|x| &x.token_type)
        .collect::<Box<_>>()
    );

    let nodes = SourceFile::parse(&tokens);
    let _action_tree = nodes.visit();
}
