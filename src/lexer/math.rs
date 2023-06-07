use logos::Logos;
use wagon_macros::inherit_from_base;
use crate::helpers::rem_first_and_last_char;

#[inherit_from_base]
enum Math {
	#[token("not")]
	Not,

	#[token("and")]
	And,

	#[token("or")]
	Or,

	#[token("(")]
	LPar,

	#[token(")")]
	RPar,

	#[token("[")]
	LBr,

	#[token("]")]
	RBr,

	#[token("**")]
	Pow,

	#[token("*")]
	Mul,

	#[token("/")]
	Div,

	#[token("//")]
	Floor,

	#[token("%")]
	Mod,

	#[token("+")]
	Add,

	#[token("-")]
	Sub,

	#[token("==")]
	Eq,

	#[token("!=")]
	Neq,

	#[token("<=")]
	Lte,

	#[token("<")]
	Lt,

	#[token(">=")]
	Gte,

	#[token(">")]
	Gt,

	#[token("in")]
	In,

	#[token("false", |_| false)]
    #[token("true", |_| true)]
    LitBool(bool),

    #[regex("(-?[0-9]+)", |lex| lex.slice().parse::<i64>().unwrap(), priority = 2)]
	LitInt(i64),

	#[regex("(-?[0-9]+\\.[0-9]+)", |lex| lex.slice().parse::<f64>().unwrap())]
	LitFloat(f64),
}

#[cfg(test)]
mod tests {

	
	use crate::helpers::assert_lex;
	use super::Math::{*};

	#[test]
	fn test_pure_math() {
		let s = "5*7.43**2 + (3 % -2)";
		let expect = &[
			Ok(LitInt(5)),
			Ok(Mul),
			Ok(LitFloat(7.43)),
			Ok(Pow), 
			Ok(LitInt(2)),
			Ok(Add), 
			Ok(LPar),
			Ok(LitInt(3)),
			Ok(Mod), 
			Ok(LitInt(-2)),
			Ok(RPar),
		];
		assert_lex(s, expect);
	}

	#[test]
	fn test_pure_bool() {
		let s = "not(true and false or x)";
		let expect = &[
			Ok(Not),
			Ok(LPar),
			Ok(LitBool(true)),
			Ok(And),
			Ok(LitBool(false)),
			Ok(Or),
			Ok(Ident("x".to_string())),
			Ok(RPar),
		];
		assert_lex(s, expect);
	}

	#[test]
	fn test_complex_combination() {
		let s = "0.5 * ((2 + 3 < 4 + x) and (x != 2) or '4' in y)";
		let expect = &[
			Ok(LitFloat(0.5)),
			Ok(Mul),
			Ok(LPar),
			Ok(LPar),
			Ok(LitInt(2)),
			Ok(Add),
			Ok(LitInt(3)),
			Ok(Lt),
			Ok(LitInt(4)),
			Ok(Add),
			Ok(Ident("x".to_string())),
			Ok(RPar),
			Ok(And),
			Ok(LPar),
			Ok(Ident("x".to_string())),
			Ok(Neq),
			Ok(LitInt(2)),
			Ok(RPar),
			Ok(Or),
			Ok(LitString("4".to_string())),
			Ok(In),
			Ok(Ident("y".to_string())),
			Ok(RPar),
		];
		assert_lex(s, expect);
	}

}