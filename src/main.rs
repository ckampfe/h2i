use lazy_static::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, hex_digit1};
use nom::*;
use std::collections::HashMap;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "hh")]
struct Options {
    /// Either a hex number like 0x0A or a positive integer like 10
    #[structopt()]
    number: String
}

type ParseResult<T> = std::result::Result<(T, Output), nom::Err<(T, nom::error::ErrorKind)>>;

#[derive(Debug)]
enum Output {
    ToHex(usize),
    ToInt(usize),
}

lazy_static! {
    static ref HASHMAP: HashMap<u8, usize> = {
        let mut m = HashMap::new();
        m.insert(b"0"[0], 0);
        m.insert(b"1"[0], 1);
        m.insert(b"2"[0], 2);
        m.insert(b"3"[0], 3);
        m.insert(b"4"[0], 4);
        m.insert(b"5"[0], 5);
        m.insert(b"6"[0], 6);
        m.insert(b"7"[0], 7);
        m.insert(b"8"[0], 8);
        m.insert(b"9"[0], 9);
        m.insert(b"A"[0], 10);
        m.insert(b"B"[0], 11);
        m.insert(b"C"[0], 12);
        m.insert(b"D"[0], 13);
        m.insert(b"E"[0], 14);
        m.insert(b"F"[0], 15);
        m
    };
}

fn parse(s: &[u8]) -> ParseResult<&[u8]> {
    let (s, out) = alt((hex_to_dec, dec_to_hex))(s)?;

    Ok((s, out))
}

fn hex_to_dec(s: &[u8]) -> ParseResult<&[u8]> {
    let (s, _) = tag("0x")(s)?;
    let (s, hex_digits) = hex_digit1(s)?;

    let digits = hex_digits
        .as_bytes()
        .iter()
        .rev()
        .enumerate()
        .map(|(i, d)| {
            let sixteen_factor = i * 16;
            if i > 0 {
                HASHMAP.get(&d).unwrap() * sixteen_factor
            } else {
                *HASHMAP.get(&d).unwrap()
            }
        });

    let number = digits.sum();

    Ok((s.as_bytes(), Output::ToInt(number)))
}

fn dec_to_hex(s: &[u8]) -> ParseResult<&[u8]> {
    let (s, digits) = digit1(s)?;
    Ok((
        s,
        Output::ToHex(
            std::str::from_utf8(digits)
                .unwrap()
                .parse::<usize>()
                .unwrap(),
        ),
    ))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::from_args();
    let (_s, output) = parse(options.number.as_bytes()).unwrap();

    match output {
        Output::ToInt(n) => println!("{}", n),
        Output::ToHex(n) => println!("{:#X}", n),
    }

    Ok(())
}
