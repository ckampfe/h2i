use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, hex_digit1};
use nom::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "h2i")]
struct Options {
    /// Either a hex number like 0x0A or a positive integer like 10
    #[structopt()]
    number: String,
}

type ParseResult<T, O> = std::result::Result<(T, O), nom::Err<(T, nom::error::ErrorKind)>>;

enum Output {
    ToHex(usize),
    ToInt(usize),
}

fn hex_byte_to_dec(byte: u8) -> usize {
    match &[byte] {
        b"0" => 0,
        b"1" => 1,
        b"2" => 2,
        b"3" => 3,
        b"4" => 4,
        b"5" => 5,
        b"6" => 6,
        b"7" => 7,
        b"8" => 8,
        b"9" => 9,
        b"A" => 10,
        b"B" => 11,
        b"C" => 12,
        b"D" => 13,
        b"E" => 14,
        b"F" => 15,
        &[_] => panic!("Byte must be a valid hex byte"),
    }
}

fn parse(s: &[u8]) -> ParseResult<&[u8], Output> {
    let (s, out) = alt((hex_to_dec, dec_to_hex))(s)?;

    Ok((s, out))
}

fn hex_to_dec(s: &[u8]) -> ParseResult<&[u8], Output> {
    let (s, _) = tag("0x")(s)?;
    let (s, hex_digits) = hex_digit1(s)?;

    let digits = hex_digits
        .as_bytes()
        .iter()
        .rev()
        .enumerate()
        .map(|(i, d)| {
            if i > 0 {
                let sixteen_factor = i * 16;
                hex_byte_to_dec(*d) * sixteen_factor
            } else {
                hex_byte_to_dec(*d)
            }
        });

    let number = digits.sum();

    Ok((s.as_bytes(), Output::ToInt(number)))
}

fn dec_to_hex(s: &[u8]) -> ParseResult<&[u8], Output> {
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
