use risp::env;
use risp::*;
use std::io::Write;
use std::*;

fn main() {
  let env = &mut env::default_env();
  loop {
    print!("risp > ");
    io::stdout().flush().unwrap();
    let expr = slurp_expr();
    match parse_eval(expr, env) {
      Ok(res) => println!("// 🔥  => {}", res),
      Err(e) => match e {
        RispErr::Reason(msg) => println!("// 🙀 => {}", msg),
      },
    }
  }
}

fn slurp_expr() -> String {
  let mut expr = String::new();
  io::stdin()
    .read_line(&mut expr)
    .expect("Failed to read line");

  expr
}
