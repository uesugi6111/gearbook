use clap::Clap;
use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
};

#[derive(Clap, Debug)]
#[clap(
    name = "my rpn program",
    version = "1.0.0",
    author = "uegusi",
    about = "calc"
)]
struct Opts {
    ///formula written in rpn
    #[clap(short, long)]
    verbose: bool,
    ///sets ths level of verbosity
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();
    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);

        run(reader, opts.verbose);
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose);
    }
}

fn run<R: BufRead>(reader: R, verbose: bool) {
    let clac = RpnCalculater::new(verbose);
    for line in reader.lines() {
        let line = line.unwrap();
        let answer = clac.eval(&line);
        println!("{}", answer);
    }
}
const MSG_PARSE_ERROR: &str = "invalid syntax";

struct RpnCalculater(bool);
impl RpnCalculater {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }
    pub fn eval(&self, formula: &str) -> i32 {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }
    pub fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
        let mut stack = Vec::new();
        while let Some(token) = tokens.pop() {
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let x = stack.pop().expect(MSG_PARSE_ERROR);
                let y = stack.pop().expect(MSG_PARSE_ERROR);

                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => panic!("invalid token"),
                };
                stack.push(res);
            }

            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }
        if stack.len() == 1 {
            stack[0]
        } else {
            panic!(MSG_PARSE_ERROR);
        }
    }
}
