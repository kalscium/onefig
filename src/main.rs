use std::fs;
use flexar::lext::Lext;
use onefig::{lexer::Token, nodes::source_file::SourceFile, conff::ConfFile};

fn main() {
    let file = fs::read_to_string("example.of").unwrap();
    let tokens = Token::tokenize(Lext::new("example.of".into(), &file));

    let nodes = SourceFile::parse(&tokens);
    let action_tree = nodes.visit();
    let conf_files = ConfFile::from_att(action_tree);
    ConfFile::serialize(&conf_files, "example.cof");
}
