use flexar::{prelude::*, compile_error::CompileError};
use crate::{lexer::Token, errors::SyntaxError};

#[derive(Debug)]
pub enum ConffType {
    Toml,
    Json,
    Nix,
}

impl ConffType {
    pub fn parse(parxt: &mut Parxt<'_, Token>) -> Result<Node<Self>, (u8, CompileError)> {
        if let Some(Token::Ident(ident)) = parxt.current() {
            let out = Ok(
                Node { position: parxt.position(), node: match ident.as_ref() {
                    "toml" => Self::Toml,
                    "json" => Self::Json,
                    "nix" => Self::Nix,
                    _ => return Err((1, compiler_error!((SY017, parxt.position()) parxt.current_token()))),
                }
            });
            parxt.advance();
            return out;
        }

        Err((0, compiler_error!((SY018, parxt.position()) parxt.current_token())))
    }
}