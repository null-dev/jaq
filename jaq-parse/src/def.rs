use super::{filter::filter, Token};
use alloc::vec::Vec;
use chumsky::prelude::*;
use jaq_syn::{Arg, Def, Main};

/// Parser for a single definition.
fn def<P>(def: P) -> impl Parser<Token, Def, Error = Simple<Token>> + Clone
where
    P: Parser<Token, Def, Error = Simple<Token>> + Clone,
{
    let arg = select! {
        Token::Ident(name) => Arg::new_filter(name),
        Token::Var(name) => Arg::new_var(name),
    };

    let defs = def.repeated().collect();

    just(Token::Def)
        .ignore_then(super::path::call(arg))
        .then_ignore(just(Token::Ctrl(':')))
        .then(defs.then(filter()).map(|(defs, body)| Main { defs, body }))
        .then_ignore(just(Token::Ctrl(';')))
        .map(|(lhs, rhs)| Def { lhs, rhs })
        .labelled("definition")
}

/// Parser for a sequence of definitions.
pub fn defs() -> impl Parser<Token, Vec<Def>, Error = Simple<Token>> + Clone {
    recursive(def).repeated().collect()
}

/// Parser for a (potentially empty) sequence of definitions, followed by a filter.
pub fn main() -> impl Parser<Token, Main, Error = Simple<Token>> + Clone {
    defs()
        .then(filter())
        .map(|(defs, body)| Main { defs, body })
}
