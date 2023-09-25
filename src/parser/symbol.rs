use super::ast::{ToAst, WagNode};
use super::{Parse, PeekLexer, ParseResult, Tokens, WagParseError};
use crate::lexer::{math::Math, productions::Productions, ident::Ident, UnsafeNext, UnsafePeek, Spannable};

use super::terminal::Terminal;
use super::assignment::Assignment;

#[derive(PartialEq, Debug, Eq, Hash)]
pub(crate) enum Symbol {
	NonTerminal(Ident),
	Assignment(Vec<Assignment>),
	Terminal(Terminal),
}

impl Parse for Symbol {
    fn parse(lexer: &mut PeekLexer) -> ParseResult<Self> where Self: Sized {
        match lexer.peek_unwrap() {
        	Tokens::ProductionToken(Productions::Identifier(_)) => {
        		if let Tokens::ProductionToken(Productions::Identifier(x)) = lexer.next_unwrap() {
        			Ok(Self::NonTerminal(x))
        		} else { 
        			Err(WagParseError::Fatal((lexer.span(), "Something went terribly wrong. Unwrapped non-identifier when should have unwrapped identifier".to_string()))) 
        		}
        	},
        	Tokens::ProductionToken(Productions::LCur) => {
                lexer.next();
                Ok(Self::Assignment(Assignment::parse_sep_end(lexer, Tokens::MathToken(Math::Semi), Tokens::MathToken(Math::RCur))?))
        	},
        	_ => Ok(Self::Terminal(Terminal::parse(lexer)?))
        }
    }
}

impl ToAst for Symbol {
    fn to_ast(self, ast: &mut super::ast::WagTree) -> super::ast::WagIx {
        match self {
            Symbol::NonTerminal(i) => ast.add_node(WagNode::Ident(i)),
            Symbol::Terminal(t) => ast.add_node(WagNode::Terminal(t)),
            Symbol::Assignment(v) => {let node = WagNode::Assignments; Self::add_vec_children(node, v, ast)},
        }
    }
}