use super::ast::ToAst;
use super::{Parse, PeekLexer, ParseResult, ParseOption, Tokens};

use crate::lexer::{math::Math, UnsafePeek};

use super::term::Term;
use super::helpers::TokenMapper;
use wagon_macros::TokenMapper;

#[derive(PartialEq, Debug, Eq, Hash, Clone)]
pub(crate) struct Sum {
	pub(crate) left: Term,
	pub(crate) cont: Option<SumP>
}

#[derive(PartialEq, Debug, Eq, Hash, Clone)]
pub(crate) struct SumP {
	pub(crate) op: Op1,
	pub(crate) right: Term,
	pub(crate) cont: Box<Option<SumP>>
}

impl Parse for Sum {

	fn parse(lexer: &mut PeekLexer) -> ParseResult<Self> {
		Ok(Self {
			left: Term::parse(lexer)?,
			cont: SumP::parse_option(lexer)?
		})
	}
}

impl ParseOption for SumP {

	fn parse_option(lexer: &mut PeekLexer) -> ParseResult<Option<Self>> where Self: Sized {
	    if let Some(op) = Op1::token_to_enum(lexer.peek_unwrap()) {
	    	lexer.next();
	    	Ok(Some(SumP { op, right: Term::parse(lexer)?, cont: Box::new(SumP::parse_option(lexer)?) }))
	    } else {
	    	Ok(None)
	    }
	}
}

#[derive(TokenMapper, PartialEq, Debug, Eq, Hash, Clone)]
pub(crate) enum Op1 {
	Add,
	Sub
}

impl ToAst for Sum {
    fn to_ast(self, ast: &mut super::ast::WagTree) -> super::ast::WagIx {
        let node = if let Some(cont) = self.cont {
        	cont.to_ast(ast)
        } else {
        	ast.add_node(super::ast::WagNode::Sum(None))
        };
        let child =  self.left.to_ast(ast);
        ast.add_edge(node, child, ());
        node
    }
}

impl ToAst for SumP {
    fn to_ast(self, ast: &mut super::ast::WagTree) -> super::ast::WagIx {
        let node = ast.add_node(super::ast::WagNode::Sum(Some(self.op)));
        let ret = if let Some(next) = *self.cont {
        	let parent = next.to_ast(ast);
        	ast.add_edge(parent, node, ());
        	parent
        } else {
        	node
        };
        let child = self.right.to_ast(ast);
        ast.add_edge(ret, child, ());
        ret
    }
}