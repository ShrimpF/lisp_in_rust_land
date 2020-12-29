use crate::*;
use std::num::ParseFloatError;

pub fn parse<'a>(tokens: &'a [String]) -> Result<(RispExp, &'a [String]), RispErr> {
	let (token, rest) = tokens
		.split_first()
		.ok_or(RispErr::Reason("could not get token".to_string()))?;

	match &token[..] {
		"(" => read_seq(rest),
		")" => Err(RispErr::Reason("unexpected `)`".to_string())),
		_ => Ok((parse_atom(token), rest)),
	}
}

fn read_seq<'a>(tokens: &'a [String]) -> Result<(RispExp, &'a [String]), RispErr> {
	let mut res: Vec<RispExp> = vec![];
	let mut xs = tokens;

	loop {
		let (next_token, rest) = xs
			.split_first()
			.ok_or(RispErr::Reason("could not find closing `)`".to_string()))?;

		if next_token == ")" {
			return Ok((RispExp::List(res), rest));
		}
		let (exp, new_xs) = parse(&xs)?;
		res.push(exp);
		xs = new_xs;
	}
}

fn parse_atom(token: &str) -> RispExp {
	match token.as_ref() {
		"true" => RispExp::Bool(true),
		"false" => RispExp::Bool(false),
		_ => {
			let potential_float: Result<f64, ParseFloatError> = token.parse();
			match potential_float {
				Ok(v) => RispExp::Number(v),
				Err(_) => RispExp::Symbol(token.to_string().clone()),
			}
		}
	}
}

pub fn parse_list_of_floats(args: &[RispExp]) -> Result<Vec<f64>, RispErr> {
	args.iter().map(|x| parse_single_float(x)).collect()
}

fn parse_single_float(exp: &RispExp) -> Result<f64, RispErr> {
	match exp {
		RispExp::Number(num) => Ok(*num),
		_ => Err(RispErr::Reason("expected a number".to_string())),
	}
}
