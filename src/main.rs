use std::fs;
use clap::Parser;
use flexar::lext::Lext;
use onefig::{lexer::Token, nodes::source_file::SourceFile, conff::ConfFile, cli::Cli};

fn main() {
    let file = fs::read_to_string("example.nf").unwrap();
    let tokens = Token::tokenize(Lext::new("example.nf".into(), &file));

    let nodes = SourceFile::parse(tokens);
    let action_tree = nodes.visit();
    let conf_files = ConfFile::from_att(action_tree);
    ConfFile::compile(&conf_files, "example.cnf");

    // drop and load
    std::mem::drop(conf_files);
    let conf_files = ConfFile::load_compiled("example.cnf");

    conf_files.iter().for_each(|x| x.generate());
}

fn _new_main() {
    let _args = Cli::parse();
}