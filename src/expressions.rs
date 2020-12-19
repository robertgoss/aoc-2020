use pest::Parser;
use itertools::Itertools;


#[derive(Parser)]
#[grammar = "expressions.pest"] // relative to src
struct ExpressionsParser;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Op {
    Plus,
    Mult
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expression {
    Num(i64),
    Expression(Box<Expression>, Vec<(Op, Expression)>)
}

use pest::iterators::Pair;

fn parse_op(pair : Pair<Rule>) -> Op {
    match pair.as_rule() {
        Rule::add => Op::Plus,
        Rule::multiply => Op::Mult,
        _ => unreachable!()
    }
}
    
fn parse_expr(pair : Pair<Rule>) -> Expression {
    match pair.as_rule() {
        Rule::num => Expression::Num(pair.as_str().parse().unwrap()),
        Rule::term => parse_expr(pair.into_inner().next().unwrap()),
        Rule::expression => {
            let mut inner = pair.into_inner();
            let expr = parse_expr(inner.next().unwrap());
            let rest : Vec<(Op, Expression)> = inner.tuples::<(_, _)>().map(
                |combine| (parse_op(combine.0), parse_expr(combine.1))
            ).collect();
            Expression::Expression(Box::new(expr), rest)
        }
        _ => unreachable!()
    }
}


impl Expression {

    pub fn from_string(string : &str) -> Option<Expression> {
        let problem = ExpressionsParser::parse(Rule::problem, string).expect(
            "unsuccessful parse"
        ).next().unwrap();
        Some(
            parse_expr(problem.into_inner().next().unwrap())
        )
    }

    pub fn compute(self : &Self) -> i64 {
        match self {
            Expression::Num(val) => *val,
            Expression::Expression(
                initial,
                rest
            ) => {
                let mut val = initial.compute();
                for (op, expr) in rest.iter() {
                    match op {
                        Op::Plus => val = val + expr.compute(),
                        Op::Mult => val = val * expr.compute()
                    };
                }
                val
            }
        }
    }

    pub fn compute_precedent(self : &Self) -> i64 {
        match self {
            Expression::Num(val) => *val,
            Expression::Expression(initial,rest) => { 
                match &rest[..] {
                    [] => initial.compute_precedent(),
                    [(op, expr)] => match op {
                        Op::Plus => initial.compute_precedent() + expr.compute_precedent(),
                        Op::Mult => initial.compute_precedent() * expr.compute_precedent()
                    },
                    _ => {
                        let index = rest.iter().enumerate().filter(
                            |(_,(op,_))| *op == Op::Mult
                        ).map(
                            |(i,_)| i 
                        ).next().unwrap_or(0);
                        let op = rest[index].0;
                        let expr1 = Expression::Expression(initial.clone(), rest[0..index].to_vec());
                        let expr2 = Expression::Expression(Box::new(rest[index].1.clone()), rest[index+1..].to_vec());
                        match op {
                            Op::Plus => expr1.compute_precedent() + expr2.compute_precedent(),
                            Op::Mult => expr1.compute_precedent() * expr2.compute_precedent()
                        }
                    }
                }
            }
        }
    }
}