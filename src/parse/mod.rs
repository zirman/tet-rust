use std::collections::HashMap;

enum Json {
    String(String),
    Number(f64),
    Object(HashMap<String, Json>),
    Array(Vec<Json>),
    Null,
    True,
    False,
}

trait Parser<A>: Sized {
    fn parse<'a, 'b>(&'a self, source: &'b str) -> Result<(A, &'b str), &'b str>;

    fn map<B, F: Fn(A) -> B>(self, f: F) -> ParserMap<A, B, Self, F> {
        ParserMap {
            p: self,
            f,
            phantom_a: PhantomData,
            phantom_b: PhantomData,
        }
    }

    fn ap<B, PB: Parser<B>>(self, pb: PB) -> ParserAp<A, B, Self, PB> {
        ParserAp {
            pa: self,
            pb,
            phantom_a: PhantomData,
            phantom_b: PhantomData,
        }
    }

    fn flat_map<B, PB: Parser<B>, F: Fn(A) -> PB>(self, f: F) -> ParserFlatMap<A, B, Self, PB, F> {
        ParserFlatMap {
            p: self,
            f,
            phantom_a: PhantomData,
            phantom_b: PhantomData,
        }
    }

    fn or<P2: Parser<A>>(self, p: P2) -> ParserOr<A, Self, P2> {
        ParserOr {
            p1: self,
            p2: p,
            phantom_a: PhantomData,
        }
    }

    fn nest<B, F: Fn(A, &str) -> Result<(B, &str), &str>>(self, f: F) -> ParserNest<A, B, Self, F> {
        ParserNest {
            p: self,
            f,
            phantom_a: PhantomData,
            phantom_b: PhantomData,
        }
    }
}

struct ParsePure<A> {
    x: A,
}

impl<A> ParsePure<A> {
    fn new(x: A) -> ParsePure<A> {
        ParsePure { x }
    }
}

impl<A: Clone> Parser<A> for ParsePure<A> {
    fn parse<'a, 'b>(&'b self, source: &'a str) -> Result<(A, &'a str), &'a str> {
        Ok((self.x.clone(), source))
    }
}

use std::marker::PhantomData;

struct ParserMap<A, B, P: Parser<A>, F: Fn(A) -> B> {
    p: P,
    f: F,
    phantom_a: PhantomData<A>,
    phantom_b: PhantomData<B>,
}

impl<A, B, P: Parser<A>, F: Fn(A) -> B> Parser<B> for ParserMap<A, B, P, F> {
    fn parse<'a, 'b>(&'b self, source: &'a str) -> Result<(B, &'a str), &'a str> {
        let result = self.p.parse(source);

        match result {
            Ok((x, source)) => Ok(((self.f)(x), source)),

            Err(error) => Err(error),
        }
    }
}

struct ParserAp<A, B, PA: Parser<A>, PB: Parser<B>> {
    pa: PA,
    pb: PB,
    phantom_a: PhantomData<A>,
    phantom_b: PhantomData<B>,
}

impl<A, B, PA: Parser<A>, PB: Parser<B>> Parser<A> for ParserAp<A, B, PA, PB> {
    fn parse<'a, 'b>(&'b self, source: &'a str) -> Result<(A, &'a str), &'a str> {
        let result = self.pa.parse(source);

        match result {
            Ok((x, source)) => match self.pb.parse(source) {
                Ok((_, source)) => Ok((x, source)),
                Err(source) => Err(source),
            },
            Err(source) => Err(source),
        }
    }
}

struct ParserFlatMap<A, B, PA: Parser<A>, PB: Parser<B>, F: Fn(A) -> PB> {
    p: PA,
    f: F,
    phantom_a: PhantomData<A>,
    phantom_b: PhantomData<B>,
}

impl<A, B, PA: Parser<A>, PB: Parser<B>, F: Fn(A) -> PB> Parser<B>
for ParserFlatMap<A, B, PA, PB, F>
{
    fn parse<'a, 'b>(&'b self, source: &'a str) -> Result<(B, &'a str), &'a str> {
        let result = self.p.parse(source);

        match result {
            Ok((x, source)) => (self.f)(x).parse(source),

            Err(x) => Err(x),
        }
    }
}

struct ParserOr<A, P1: Parser<A>, P2: Parser<A>> {
    p1: P1,
    p2: P2,
    phantom_a: PhantomData<A>,
}

impl<A, P1: Parser<A>, P2: Parser<A>> Parser<A> for ParserOr<A, P1, P2> {
    fn parse<'a, 'b>(&'b self, source: &'a str) -> Result<(A, &'a str), &'a str> {
        let result = self.p1.parse(source);

        match result {
            Ok(x) => Ok(x),
            Err(_) => self.p2.parse(source),
        }
    }
}

struct ParserNest<A, B, PA: Parser<A>, F: Fn(A, &str) -> Result<(B, &str), &str>> {
    p: PA,
    f: F,
    phantom_a: PhantomData<A>,
    phantom_b: PhantomData<B>,
}

impl<A, B, PA: Parser<A>, F: Fn(A, &str) -> Result<(B, &str), &str>> Parser<B>
for ParserNest<A, B, PA, F>
{
    fn parse<'a, 'b>(&'b self, source: &'a str) -> Result<(B, &'a str), &'a str> {
        let result = self.p.parse(source);

        match result {
            Ok((x, source)) => (self.f)(x, source),
            Err(x) => Err(x),
        }
    }
}

struct ParseChar {
    c: char,
}

impl ParseChar {
    fn new(c: char) -> ParseChar {
        ParseChar { c }
    }
}

impl Parser<char> for ParseChar {
    fn parse<'a, 'b>(&'a self, source: &'b str) -> Result<(char, &'b str), &'b str> {
        match source.chars().next() {
            Some(c) => {
                if c == self.c {
                    Ok((self.c, &source[1..]))
                } else {
                    Err(source)
                }
            }

            None => Err(source),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Group {
    None,
    More(Box<Group>),
}

fn parse_nested_parens(source: &str) -> Result<(Group, &str), &str> {
   ParseChar::new('(')
       .nest(|_, source| parse_nested_parens(source))
       .ap(ParseChar::new(')'))
       .map(|x| Group::More(Box::new(x)))
       .or(ParsePure::new(Group::None))
       .parse(source)
}
