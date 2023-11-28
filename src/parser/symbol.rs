use crate::parser::Span;
use std::fmt::Display;
use std::write;

use super::helpers::between_sep;
use super::ast::{ToAst, WagNode};
use super::{Parse, PeekLexer, ParseResult, Tokens, SpannableNode};
use crate::lexer::{math::Math, productions::Productions, UnsafePeek};

use super::terminal::Terminal;
use super::assignment::Assignment;
use super::Ident;

#[cfg(test)]
use wagon_macros::new_unspanned;

#[derive(PartialEq, Debug, Eq, Hash, Clone)]
#[cfg_attr(test, new_unspanned)]
pub(crate) enum Symbol {
	NonTerminal(SpannableNode<Ident>, Vec<SpannableNode<Ident>>),
	Assignment(Vec<SpannableNode<Assignment>>),
	Terminal(SpannableNode<Terminal>),
    Epsilon
}

impl Parse for Symbol {
    fn parse(lexer: &mut PeekLexer) -> ParseResult<Self> where Self: Sized {
        match lexer.peek_unwrap() {
        	Tokens::ProductionToken(Productions::Identifier(_)) => {
                let ident = SpannableNode::parse(lexer)?;
                let args = if let Some(Ok(Tokens::ProductionToken(Productions::LPar))) = lexer.peek() {
                    between_sep(lexer, Tokens::ProductionToken(Productions::LPar), Tokens::ProductionToken(Productions::RPar), Tokens::ProductionToken(Productions::Comma))?
                } else {
                    Vec::new()
                };
    			Ok(Self::NonTerminal(ident, args))
        	},
        	Tokens::ProductionToken(Productions::LCur) => {
                lexer.next();
                Ok(Self::Assignment(SpannableNode::parse_sep_end(lexer, Tokens::MathToken(Math::Semi), Tokens::MathToken(Math::RCur))?))
        	},
        	_ => Ok(Self::Terminal(SpannableNode::parse(lexer)?))
        }
    }
}

impl Default for Symbol {
    fn default() -> Self {
        Self::Epsilon
    }
}

impl Symbol {

    pub(crate) fn is_terminal(&self) -> bool {
        matches!(self, Self::Terminal(..) | Self::Assignment(..) | Self::Epsilon)
    }

    pub(crate) fn is_assignment(&self) -> bool {
        matches!(self, Self::Assignment(..))
    }

    pub(crate) fn simple_terminal(ident: &str) -> Self {
        Self::Terminal(SpannableNode::new(Terminal::LitString(ident.to_string()), 0..ident.len()))
    }

    pub (crate) fn simple_ident(ident: &str) -> Self {
        Self::NonTerminal(SpannableNode::new(Ident::Unknown(ident.to_string()), 0..ident.len()), Vec::new())
    }

    pub(crate) fn simple_ident_spanned(ident: &str, span: Span) -> SpannableNode<Self> {
        SpannableNode::new(Self::NonTerminal(SpannableNode::new(Ident::Unknown(ident.to_string()), span.clone()), Vec::new()), span)
    }
}

impl ToAst for Symbol {
    fn to_ast(self, ast: &mut super::ast::WagTree) -> super::ast::WagIx {
        match self {
            Symbol::NonTerminal(i, _) => ast.add_node(WagNode::Ident(i.into_inner())),
            Symbol::Terminal(t) => ast.add_node(WagNode::Terminal(t.into_inner())),
            Symbol::Assignment(v) => {let node = WagNode::Assignments; Self::add_vec_children(node, v, ast)},
            Symbol::Epsilon => ast.add_node(WagNode::Empty)
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::NonTerminal(i, _) => write!(f, "{}", i),
            Symbol::Assignment(i) => write!(f, "{}", i.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("; ")),
            Symbol::Terminal(i) => write!(f, "{}", i),
            Symbol::Epsilon => write!(f, "ε"),
        }
    }
}