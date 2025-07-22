use blarg::{CommandLineParser, GeneralParser, Parameter, Scalar};
use random_string::generate;
use rand::seq::SliceRandom;
use rand::rng;

#[derive(Debug, PartialEq, Eq)]
pub struct Params {
    length: u8,
    num_special: u8,
    num_digits: u8,
    num_rows: u8,
    num_columns: u8,
}

impl Params {
    fn init() -> Self {
        Self {
            length: 12,
            num_special: 1,
            num_digits: 2,
            num_rows: 1,
            num_columns: 1,
        }
    }
}

fn main() {
    let params = parse();
    let length: u8 = params.length;
    let num_special: u8 = params.num_special;
    let num_digits: u8 = params.num_digits;
    let num_rows: u8 = params.num_rows;
    let num_columns: u8 = params.num_columns;

    if num_special + num_digits > length {
        panic!(
            "A password of length {} cannot fit {} digits and {} special characters",
            length, num_digits, num_special
        );
    }

    for _r in 0..num_rows {
        for _c in 0..num_columns {
            let alpha_upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
            let alpha_lower = "abcdefghijklmnopqrstuvwxyz";
            let alphas = format!("{}{}", alpha_upper, alpha_lower);
            let digits = "1234567890";
            let special = "~!@#$%^&*()-_=+{}[];:";

            let password_alphas = generate(usize::from(length - num_special - num_digits), alphas);
            let password_digits = generate(usize::from(num_digits), digits);
            let password_special = generate(usize::from(num_special), special);

            let password = shuffle_string(format!("{}{}{}", password_alphas, password_digits, password_special));

            print!("{} ", password);
        }
        println!("");
    }
}

// Configure and execute the parser against `env::args`.
fn parse() -> Params {
    parse_tokens(|parser: GeneralParser| Ok(parser.parse()))
}
 
// Unit-testable function to configure the parser and execute it against the specified
fn parse_tokens(parse_fn: impl FnOnce(GeneralParser) -> Result<(), i32>) -> Params {
    let mut params = Params::init();

    let clp = CommandLineParser::new("organization");
    let parser = clp
        .add(Parameter::option(
            Scalar::new(&mut params.length),
            "length",
            Some('l'),
        ).help("The length of the generated password(s)"))
        .add(Parameter::option(
            Scalar::new(&mut params.num_special),
            "special",
            Some('s'),
        ).help("Number of special characters each password should contain"))
        .add(Parameter::option(
            Scalar::new(&mut params.num_digits),
            "digits",
            Some('d'),
        ).help("Number of digits each password should contain"))
        .add(Parameter::option(
            Scalar::new(&mut params.num_rows),
            "rows",
            Some('r'),
        ).help("Number of rows to generate"))
        .add(Parameter::option(
            Scalar::new(&mut params.num_columns),
            "columns",
            Some('c'),
        ).help("Number of columns to generate"))
        .build();

    // The parse_fn signature is a `Result`.
    // However, since `GeneralParser::parse` does not return an error (it uses `std::process::exit` under the hood), the `Err` case is only reached via test.
    parse_fn(parser).expect("test-reachable-only");
    params
}
 
// Shuffles the characters of a string slice and returns a new String.
fn shuffle_string(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let mut rng = rng();
    chars.shuffle(&mut rng);
    let shuffled_s: String = chars.into_iter().collect();
    
    shuffled_s
}

