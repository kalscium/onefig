use std::fs;
use clap::Parser;
use flexar::lext::Lext;
use onefig::{lexer::Token, nodes::source_file::SourceFile, conff::ConffTree, cli::Cli};

fn main() {
    let file = fs::read_to_string("example.nf").unwrap();
    let tokens = Token::tokenize(Lext::new("example.nf".into(), &file));

    let nodes = SourceFile::parse(tokens);
    let action_tree = nodes.visit();
    let conff_tree = ConffTree::from_att(action_tree);
    conff_tree.compile("example.cnf");

    // drop and load
    std::mem::drop(conff_tree);
    let conff_tree = ConffTree::load_compiled("example.cnf");

    conff_tree.conf_files.iter().for_each(|x| x.generate());
}

fn _new_main() {
    let _args = Cli::parse();
}