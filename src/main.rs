use std::fs;
use flexar::{lext::Lext, parxt::Parxt};
use onefig::{lexer::Token, nodes::stmt::Stmt};

fn main() {
    let file = fs::read_to_string("example.of").unwrap();
    let tokens = Token::tokenize(Lext::new("example.of".into(), &file));
    println!("{:?}", tokens.iter()
        .map(|x| &x.token_type)
        .collect::<Box<_>>()
    );

    let _ = Stmt::parse(&mut Parxt::new(&tokens)).map_err(|x| x.1.throw::<()>());
}
