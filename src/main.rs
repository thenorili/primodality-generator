use clap::{Arg, App};
use std::str::FromStr;
use primodality_generator::menu as menu;
use primodality_generator::error as error;
use primodality_generator::uinput as uinput;
use primodality_generator::output;
use primodality_generator::uinput::Config as Config;

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

    let ini = Config { from: from, to: to, poly: poly, };

    fn input_validation(input: &str, poly: bool) -> Result<u32, error::Error> {
        let input_int = uinput::parse(String::from(input))?;
        let input_val = uinput::check(input_int)?;
        if poly == false {
            let input_pri = uinput::check_prime(input_val)?;
            return Ok(input_pri)
        } else { return Ok(input_val) }
    }
    
    let mut num: u32;

    if args.is_present("input") {
        let input = args.value_of("input").unwrap();
        num = match input_validation(input, ini.poly) {
            Ok(number) => number,
            Err(err) => { 
                eprint!("{}", err);
                menu::dialog(ini.poly)
            },
        };
    } else { num = menu::dialog(ini.poly); }

    println!("Working...");
    std::process::exit(match primodality_generator::output::make_scl(num, ini) {
        Ok(()) => 0,
        Err(e) => {
            eprint!("{}", e);
            1
        }
    });
}
