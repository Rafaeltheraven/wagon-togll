use crate::lexer::Spannable;
use crate::firstpass::{FirstPassState, FirstPassResult, WagCheckError};

use super::ast::{WagTree, WagNode, ToAst, WagIx};
use super::{Parse, ParseResult, PeekLexer, Rewrite, SpannableNode};
use super::metadata::Metadata;
use super::rule::Rule;
use indexmap::IndexMap;

#[cfg(test)]
use wagon_macros::new_unspanned;

#[derive(PartialEq, Debug, Eq, Hash)]
#[cfg_attr(test, new_unspanned)]
pub(crate) struct Wag {
	pub(crate) metadata: Metadata,
	pub(crate) grammar: Vec<SpannableNode<Rule>>,
}

impl Parse for Wag {
    fn parse(lexer: &mut PeekLexer) -> ParseResult<Self> {
        let metadata = Metadata::parse(lexer)?;
        let mut grammar = Vec::new();
        while lexer.peek().is_some() {
        	grammar.push(SpannableNode::parse(lexer)?);
        }
        Ok(Self {metadata, grammar})
    }
}

impl ToAst for Wag {

    fn to_ast(self, ast: &mut WagTree) -> WagIx {
        let node = ast.add_node(WagNode::Root(self.metadata));
        for child in self.grammar {
            let child_ix = child.to_ast(ast);
            ast.add_edge(node, child_ix, ());
        }
        node
    }
}

impl Rewrite<()> for Wag {

    fn rewrite(&mut self, depth: usize, state: &mut FirstPassState) -> FirstPassResult<()> {
        fn handle_conflict(mut new_rule: SpannableNode<Rule>, map: &mut IndexMap<String, SpannableNode<Rule>>) -> FirstPassResult<()>{ // Combine rules for the same ident into a single rule
            let ident = match &new_rule.node {
                Rule::Analytic(s, ..) | Rule::Generate(s, ..) => s.clone(),
                Rule::Import(..) | Rule::Exclude(..) => todo!(),
            };
            if let Some(orig) = map.get_mut(&ident) {
                match (&mut orig.node, &mut new_rule.node) {
                    (Rule::Analytic(_, args1, v1), Rule::Analytic(_, args2, v2)) | (Rule::Generate(_, args1, v1), Rule::Generate(_, args2, v2)) => {
                        if args1 == args2 {
                            v1.extend(std::mem::take(v2));
                        } else {
                            return Err(WagCheckError::DisparateParameters { terminal: ident, offender: args1.to_owned(), expected: args2.to_owned(), span: new_rule.span()});
                        }
                    },
                    _ => {map.insert(ident, new_rule);}
                }
            } else {
                 map.insert(ident, new_rule);
            };
            Ok(())
        }
        let rules = std::mem::take(&mut self.grammar);
        let mut map: IndexMap<String, SpannableNode<Rule>> = IndexMap::with_capacity(rules.len());
        for mut rule in rules {
            for new_rule in rule.rewrite(depth, state)? {
                handle_conflict(new_rule, &mut map)?;
            }
            handle_conflict(rule, &mut map)?
        }
        self.grammar.extend(map.into_values());
        Ok(())
    }

}