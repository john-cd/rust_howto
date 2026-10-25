// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A calculator in two stages: a `winnow` lexer that produces zero-copy
//! tokens with byte spans, then a hand-written precedence-climbing parser
//! that builds and evaluates an expression tree.

use std::ops::Range;

use winnow::Parser as _;
use winnow::ascii::digit1;
use winnow::ascii::multispace0;
use winnow::combinator::alt;
use winnow::combinator::eof;
use winnow::combinator::opt;
use winnow::combinator::preceded;
use winnow::combinator::repeat_till;
use winnow::error::ContextError;
use winnow::prelude::*;
use winnow::stream::LocatingSlice;
use winnow::token::one_of;
use winnow::token::take_while;

/// A lexical token. Identifiers borrow from the input.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Token<'a> {
    Number(f64),
    Ident(&'a str),
    Op(char),
    LParen,
    RParen,
}

/// A token and the byte range it occupies in the input.
type Spanned<'a> = (Token<'a>, Range<usize>);

type Input<'a> = LocatingSlice<&'a str>;

/// Parses one token.
fn token<'a>(input: &mut Input<'a>) -> ModalResult<Token<'a>, ContextError> {
    alt((
        (digit1, opt(('.', digit1)))
            .take()
            .try_map(str::parse)
            .map(Token::Number),
        take_while(1.., |c: char| c.is_alphabetic() || c == '_')
            .map(Token::Ident),
        one_of(['+', '-', '*', '/', '^']).map(Token::Op),
        '('.value(Token::LParen),
        ')'.value(Token::RParen),
    ))
    .parse_next(input)
}

/// Splits `source` into tokens, skipping whitespace.
fn lex(source: &str) -> Result<Vec<Spanned<'_>>, String> {
    let mut input = LocatingSlice::new(source);
    let (tokens, _): (Vec<_>, _) = repeat_till(
        0..,
        preceded(multispace0, token.with_span()),
        preceded(multispace0, eof),
    )
    .parse_next(&mut input)
    .map_err(|_| {
        // `input` still points just past the last token that lexed.
        let consumed = source.len() - input.len();
        let offset = consumed
            + source[consumed..]
                .find(|c: char| !c.is_whitespace())
                .unwrap_or(0);
        format!("unexpected input at byte {offset}")
    })?;
    Ok(tokens)
}

/// The expression tree.
#[derive(Debug, PartialEq)]
enum Expr<'a> {
    Number(f64),
    Var(&'a str),
    Neg(Box<Self>),
    Binary(Box<Self>, char, Box<Self>),
}

/// Parser over a token vector; `pos` is the next token to read.
struct ExprParser<'a> {
    tokens: Vec<Spanned<'a>>,
    pos: usize,
    end: usize,
}

impl<'a> ExprParser<'a> {
    fn new(tokens: Vec<Spanned<'a>>, end: usize) -> Self {
        Self {
            tokens,
            pos: 0,
            end,
        }
    }

    fn peek(&self) -> Option<&Spanned<'a>> {
        self.tokens.get(self.pos)
    }

    /// Byte offset of the next token, or the end of the input.
    fn offset(&self) -> usize {
        self.peek().map_or(self.end, |(_, span)| span.start)
    }

    /// Binding power: (left, right). Right-associative operators have a
    /// lower right power.
    fn binding_power(op: char) -> (u8, u8) {
        match op {
            '+' | '-' => (1, 2),
            '*' | '/' => (3, 4),
            '^' => (6, 5),
            _ => (0, 0),
        }
    }

    /// Precedence climbing: parse operators that bind tighter than `min_bp`.
    fn expr(&mut self, min_bp: u8) -> Result<Expr<'a>, String> {
        let mut lhs = self.atom()?;
        while let Some((Token::Op(op), _)) = self.peek().cloned() {
            let (left, right) = Self::binding_power(op);
            if left < min_bp {
                break;
            }
            self.pos += 1;
            let rhs = self.expr(right)?;
            lhs = Expr::Binary(Box::new(lhs), op, Box::new(rhs));
        }
        Ok(lhs)
    }

    /// A number, variable, parenthesized expression or unary minus.
    fn atom(&mut self) -> Result<Expr<'a>, String> {
        let offset = self.offset();
        let Some((tok, _)) = self.peek().cloned() else {
            return Err(format!("unexpected end of input at byte {offset}"));
        };
        self.pos += 1;
        match tok {
            Token::Number(n) => Ok(Expr::Number(n)),
            Token::Ident(name) => Ok(Expr::Var(name)),
            // Unary minus binds looser than `^`: -2^2 == -4.
            Token::Op('-') => Ok(Expr::Neg(Box::new(self.expr(5)?))),
            Token::LParen => {
                let inner = self.expr(0)?;
                match self.peek() {
                    Some((Token::RParen, _)) => {
                        self.pos += 1;
                        Ok(inner)
                    }
                    _ => Err(format!("expected ')' at byte {}", self.offset())),
                }
            }
            other => Err(format!("unexpected {other:?} at byte {offset}")),
        }
    }
}

/// Parses `source` into an expression tree.
fn parse(source: &str) -> Result<Expr<'_>, String> {
    let tokens = lex(source)?;
    let mut parser = ExprParser::new(tokens, source.len());
    let expr = parser.expr(0)?;
    if parser.peek().is_some() {
        return Err(format!("trailing input at byte {}", parser.offset()));
    }
    Ok(expr)
}

/// Evaluates an expression; `pi` is the only variable.
fn eval(expr: &Expr<'_>) -> Result<f64, String> {
    Ok(match expr {
        Expr::Number(n) => *n,
        Expr::Var("pi") => std::f64::consts::PI,
        Expr::Var(name) => return Err(format!("unknown variable `{name}`")),
        Expr::Neg(e) => -eval(e)?,
        Expr::Binary(l, op, r) => {
            let (l, r) = (eval(l)?, eval(r)?);
            match op {
                '+' => l + r,
                '-' => l - r,
                '*' => l * r,
                '/' => l / r,
                '^' => l.powf(r),
                _ => return Err(format!("unknown operator `{op}`")),
            }
        }
    })
}

/// Parses and evaluates a formula.
fn calculate(source: &str) -> Result<f64, String> {
    eval(&parse(source)?)
}

fn main() {
    for source in ["1 + 2 * 3", "2 ^ 3 ^ 2", "-2 ^ 2", "(1 + 2) * 3", "1 +"] {
        match calculate(source) {
            Ok(value) => println!("{source} = {value}"),
            Err(message) => println!("{source}: error: {message}"),
        }
    }
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precedence_and_associativity() {
        assert_eq!(calculate("1 + 2 * 3"), Ok(7.0));
        assert_eq!(calculate("2 ^ 3 ^ 2"), Ok(512.0));
        assert_eq!(calculate("-2 ^ 2"), Ok(-4.0));
        assert_eq!(calculate("(1 + 2) * 3"), Ok(9.0));
        assert_eq!(calculate("10 - 4 - 3"), Ok(3.0));
    }

    #[test]
    fn tokens_carry_spans() {
        let tokens = lex("12 + x").unwrap();
        assert_eq!(tokens[0], (Token::Number(12.0), 0..2));
        assert_eq!(tokens[2], (Token::Ident("x"), 5..6));
    }

    #[test]
    fn errors_report_positions() {
        assert_eq!(
            calculate("1 +"),
            Err("unexpected end of input at byte 3".to_owned())
        );
        assert_eq!(
            calculate("1 + $"),
            Err("unexpected input at byte 4".into())
        );
        assert_eq!(calculate("(1 + 2"), Err("expected ')' at byte 6".into()));
        assert_eq!(calculate("1 2"), Err("trailing input at byte 2".into()));
        assert_eq!(calculate("y"), Err("unknown variable `y`".into()));
    }
}
