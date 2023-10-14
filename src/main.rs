use std::fs;
use flexar::lext::Lext;
use onefig::lexer::Token;

fn main() {
    let file = fs::read_to_string("example.of").unwrap();
    let tokens = Token::tokenize(Lext::new("example.of".into(), &file));
    println!("{:?}", tokens.iter()
        .map(|x| &x.token_type)
        .collect::<Box<_>>()
    );
}
