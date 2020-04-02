use clap::{Arg, App};
use std::str::FromStr;

struct Config {
    from: u8,
    to: u8,
    poly: bool,
} 

fn main() {
    let args = clap::App::new("Primodality_Generator")
        .version("1.1.0")
        .author("Nori Li <www.github.com/thenorili>")
        .about("Generates just intonation .scl files for a given integer denominator.")
        .arg(
            Arg::with_name("input")
                .help("Sets denominator directly, skipping the menu.")
                .index(1),
            )
        .arg(
            Arg::with_name("from")
                .help("\
                            Sets the minimum numerator as a factor of n.\n\
                            Valid values are 0-254. Default value is 1.")
                .short("f")
                .long("from")
                .takes_value(true)
                .default_value("1")
                .validator(check_min),
            )
        .arg(
            Arg::with_name("to")
                .help("\
                            Sets the maximum numerator as a factor of n.\n\
                            Valid values are 1-255. Default value is 8.")
                .short("t")
                .long("to")
                .takes_value(true)
                .default_value("8")
                .validator(check_max),
            )
        .arg(
            Arg::with_name("poly")
                .help("Permits nonprime denominators.")
                .short("p")
                .long("poly"),
            )
            .get_matches();

    fn is_u8(input: String) -> Result<u8, String> {
        if let Ok(num) = u8::from_str(&input) {
            Ok(num)
        } else {
            Err(String::from("Couldn't parse your range. Using default instead."))
        }
    }

    fn check_min(min: String) -> Result<(), String> {
        let num_or_err = is_u8(min);
        match num_or_err {
            Ok(num) => {
                if num < 255 { return Ok(()) } 
                else { return Err(String::from("Minimum can't be 255.")) } },
            Err(err) => return Err(err),
        }
    }

    fn check_max(max: String) -> Result<(), String> {
        let num_or_err = is_u8(max);
        match num_or_err {
            Ok(num) => {
                if num > 0 { return Ok(()) }
                else { return Err(String::from("Maximum can't be 0.")) } },
            Err(err) => return Err(err),
        }
    }
    let poly = false;
    if args.is_present("poly") { let poly = true; }
    let from = u8::from_str(args.value_of("from").unwrap()).unwrap();
    let to = u8::from_str(args.value_of("to").unwrap()).unwrap();

    let ini = crate::Config { from: from, to: to, poly: poly, };

    if args.is_present("input") { let input = args.value_of("input").unwrap(); }

    fn input_validation(input: &str, poly: bool) -> Result<u32, crate::error:Error> {
        let input_int = crate::uinput::parse(input)?;
        let input_val = crate::uinput::parse(input_int)?;
        if poly = false {
            let input_pri = crate::uinput::check_prime(input_val)?;
            return input_pri
        } else { return input_val }
    }

    if args.is_present("input") {
        let num_or_err = match input_validation(input, ini.poly) {
            Ok(num) => num,
            Err(err) => { 
                eprint!("{}", err);
                let num = crate::menu::dialog(ini.poly);
            },
        };
    } else { let num = crate::menu::dialog(ini.poly); }

    println!("Working...");
    std::process::exit(match output::make_scl(num, ini) {
        Ok(()) => 0,
        Err(e) => {
            eprint!("{}", e);
            1
        }
    });
}

mod menu {

    use std::io::{self, Write};
    use crate::uinput;
    use std::process;

    pub fn dialog(poly: bool) -> u32 {
        println!("Please input an integer to generate its polyprimodal .scl file.");
        let num = loop {
            print!(">>> ");
            io::stdout().flush().unwrap();
            let raw_input = uinput::get_parse_check();
            if poly == false {
                if let Ok(raw_input) = raw_input { 
                    let raw_input = uinput::check_prime(raw_input);
                }
            }
            match raw_input {
                Ok(input) => { break input }
                Err(e) => {
                    eprint!("{} ", e);
                    println!("Input (r) to reset or the any key to quit the program");
                    let choice = uinput::get();
                    match choice {
                        Ok(d) => {
                            if d.trim() == "r" { (); 
                            } else { process::exit(0); }
                        }
                        Err(e) => {
                            eprint!("{}", e);
                            process::exit(1);
                        }
                    }
                }
            }
        };
        return num
    }
}

mod error {
    
    use std::fmt;
    use std::num;
    use std::io;

    #[derive(Debug)]
    pub enum Error {
        Io(io::Error),
        Parse(num::ParseIntError),
        TooSmall,
        NotPrime
    }   

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Error::Io(ref err) => write!(f, "IO error: {}", err),
                Error::Parse(ref err) => write!(f, "Parse error: {}", err),
                Error::TooSmall => write!(f, "IO error: You can do better. Aim higher!"),
                Error::NotPrime => write!(f, "IO error: Curiously, your input is not prime."),
            }
        }
    }

    impl From<io::Error> for Error {
        fn from(err: io::Error) -> Error {
            Error::Io(err)
        }
    }

    impl From<num::ParseIntError> for Error {
        fn from(err: num::ParseIntError) -> Error {
            Error::Parse(err)
        }
    }
}


mod uinput {
    use std::io;
    use crate::error::Error;
    use primapalooza::is_prime;

    pub fn get() -> Result<String, Error> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input)
    }

    pub fn parse(input: String) -> Result<u32, Error> {
        let output = input.trim().parse::<u32>()?;
        Ok(output)
    }

    pub fn check(input: u32) -> Result<u32, Error> {
        if input <= 2 { 
            Err(Error::TooSmall)
        } else {
          Ok(input)
        }
    }
    
    pub fn check_prime(input: u32) -> Result<u32, Error> {
        if is_prime(input as usize) {
            Ok(input)
        } else {
            Err(Error::NotPrime)
        }
    }

    pub fn get_parse_check() -> Result<u32, Error> {
        let input = get()?;
        let output = parse(input)?;
        let output = check(output)?;
        Ok(output)
    }

    pub fn val_parse_check(input: Option<&str>) -> Result<u32, Error> {
        if let Some(number) = input {
            let thruput = parse(String::from(number))?;
            let output = check(thruput)?;
            Ok(output)
        } else { Err(Error::Io) }
    }
}


mod output {
    use num_rational::Ratio;
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    use crate::Config;
    
    pub fn make_scl(n: u32, ini: Config) -> std::io::Result<()> {
        if ini.poly == true {
            let nom = "polyprimodality";
        } else {
            let nom = "primodality";
        }
        let name = format!("{}-{}.scl", nom, n);
        let mut scl = OpenOptions::new()
            .append(true)
            .create(true)
            .open(name)?;
        writeln!(scl, "{} {} scale {}n through {}n", n, nom, ini.from, ini.to)?;
        writeln!(scl, "    {}", (((ini.to - ini.from) * n) - 1))?;
        let mut n = (ini.from * n) + 1;
        while i != (ini.to * n) {
            writeln!(scl, "{}", Ratio::new(i, n))?;
            i += 1;
        };
        println!("Complete!");
        Ok(())
    }
}
