use clap::{error as clap_error, CommandFactory, Parser};
use rand::{rngs::ThreadRng, Rng};
use regex::{Regex, RegexBuilder};
use std::{io::stdin, process::exit};
use text_io::read;

// A very sus char
const SUS_CHAR: char = 'ඞ';

// Upper limit for the sussiness parameter
const DEFAULT_MAX_SUSSINESS: u32 = 10;
// How ඞ sus ඞ your string is by default
const DEFAULT_SUSSINESS: u32 = 1;

// In order to make some parts of the text appear bold or underlined, we use ANSI escape codes
const SHORT_ABOUT: &'static str = "\x1B[1mA very ඞ sus ඞ program\x1B[0m";
const LONG_ABOUT: &'static str = concat!(
    "\x1B[1m\x1B[4mSussifies an input string\n\x1B[0m",
    "",
    "For each character on the input string, there is a n/m chance of it",
    "being replaced with ඞ, where n = 1 unless set by the user using",
    "--sus option and m = 10 unless set using the --max-sus option\n",
    "\n",
    "\x1B[1m\x1B[4mULTRA ඞ SUS ඞ MODE™️\x1B[0m",
    "\n",
    "\x1b[4mPlease be very careful when you are using the ULTRA ඞ SUS ඞ MODE™️. ",
    "Incorrect usage of it can result in mass hysteria, false vacuum decay or/and an ധK-Class scenario\x1B[24m\n",
    "\n",
    "Replaces all occurences of the word \"sus\" (lowercase and uppercase) with ඞ. ",
    "Furthermore, it displays a funni message if the total number of ඞ occurences after ",
    "the processing of the input string is over sum up to either 69 or 420"
);

#[derive(Parser)]
#[command(
    version,
    about = SHORT_ABOUT,
    long_about = SHORT_ABOUT.to_owned() + "\n\n" + LONG_ABOUT
)]
struct Args {
    /// Input string to ඞ sussify ඞ
    #[arg(value_name = "string")]
    input: Option<String>,

    /// How ඞ sus ඞ you want the output string to be
    #[arg(short, long = "sus", default_value_t = DEFAULT_SUSSINESS, value_name = "sussiness")]
    sussiness: u32,

    /// Set a new upper limit for how ඞ sus ඞ the output string can be
    #[arg(long = "max-sus", default_value_t = DEFAULT_MAX_SUSSINESS, value_name = "max-sussiness")]
    max_sussiness: u32,

    /// Activate the ULTRA ඞ SUS ඞ MODE™️. I sure hope you know what you are doing
    #[arg(short = 'U', long = "ultra-sus")]
    ultra_sus_mode: bool,

    /// Skip all safety checks when running ULTRA ඞ SUS ඞ MODE™️
    ///
    /// WARNING: This will make the program ultra ඞ sus ඞ.
    /// By running this you accept the possibility that a specialised Interpol unit
    /// may be deployed near your location because you are being ultra ඞ sus ඞ
    #[arg(short = 'y')]
    skip_checks: bool,
}

fn value_validation_err(msg: impl std::fmt::Display) -> ! {
    let mut cmd = Args::command();
    cmd.error(clap_error::ErrorKind::ValueValidation, msg)
        .exit()
}

fn sussify(mut input: String, args: &Args, rng: &mut ThreadRng) -> String {
    // Before doing anything, LOCATE ALL ANSI CONTROL/ESCAPE CODES
    //
    // Note: I am pretty sure this RegEx doesn't catch every ANSI escape code.
    // If you find such a case, please file an issue at https://github.com/Oakchris1955/rust-amongify/issues
    let re = Regex::new(r"\x1B\[.*[A-Za-z]").unwrap();
    let ranges = re.find_iter(&input).map(|f| f.range()).collect::<Vec<_>>();

    // In case we are in ULTRA ඞ SUS ඞ MODE™️, prompt the user
    // just to make sure they didn't trigger it by accident
    if args.ultra_sus_mode {
        if !args.skip_checks {
            print!(concat!(
                "Warning: The program is about to run in ULTRA ඞ SUS ඞ MODE™️.\n",
                "Are you sure you wanna continue? [y/N]: "
            ));
            let input: String = read!("{}\n");
            if input.trim().to_lowercase() != "y" {
                exit(1)
            }
        }

        // Once we are done with safety checks, we can start processing the string
        // in ULTRA ඞ SUS ඞ MODE™️ before processing it as we normally would
        let re = RegexBuilder::new(r"sus")
            .case_insensitive(true)
            .build()
            .unwrap();

        input = re.replace_all(&input, SUS_CHAR.to_string()).to_string();
    }

    // Loop through each character on input string
    // and replace them with SUS_CHAR, according to the sussiness parameter
    // Note: special characters, such as carriage return (CR) won't be replaced
    input = input
        .chars()
        .enumerate()
        .map(|(i, char)| {
            if rng.gen_ratio(args.sussiness, args.max_sussiness)
                && !char.is_control()
                && char != SUS_CHAR
                && !ranges.iter().any(|range| range.contains(&i))
            {
                SUS_CHAR
            } else {
                char
            }
        })
        .collect();

    // Again, if we are in ULTRA ඞ SUS ඞ MODE™️, and there are exactly 69 or 420 ඞ characters,
    // just print a text indicating something is pretty sus and exit (code 0)
    if args.ultra_sus_mode {
        match input.matches(&SUS_CHAR.to_string()).count() {
            69 => {
                println!("You are too sus to be sus");
                exit(0)
            }
            420 => {
                println!("You are WAY TOO ඞ SUS ඞ to be ඞ SUS ඞ");
                println!(concat!(
                    "Note: as an added bonus, if you get this message 69 times in a row, ",
                    "there is a 10^-420 chance that mass hysteria surrounding the video-game-that-shall-not-be-named will happen, ",
                    "a 10^-690 that the universe will undergo false-vacuum decay, ",
                    "and a 10^-69420 that an ധK-Class scenario will occur"
                ));
                exit(0)
            }
            _ => (),
        }
    }

    input
}

fn read_stdin_to_line() -> String {
    let mut output = String::new();
    stdin().read_line(&mut output).unwrap();

    output
}

fn main() {
    // Parse arguments
    let args = Args::parse();

    // Retrieve a lazily-initialized thread-local RNG
    let mut rng = rand::thread_rng();

    // Do some bound checking
    if args.sussiness > args.max_sussiness {
        value_validation_err(format!(
            "Provided sussiness value {} is out of range (max {})",
            args.sussiness, args.max_sussiness
        ))
    }

    if args.max_sussiness <= 0 {
        value_validation_err(concat!(
            "Looks like you are pretty ඞ sus ඞ. \n",
            "You tried setting the denominator of the n/m division to 0, didn't you?"
        ))
    }

    let mut read_from_input = || loop {
        let line = read_stdin_to_line();
        if line.len() == 0 {
            println!("");
            break;
        }
        print!("{}", sussify(line, &args, &mut rng));
    };

    // If not input string was supplied or it is equal to "-", read from stdin until EOF
    match args.input.clone() {
        Some(input) => match input.trim() {
            "-" => read_from_input(),
            _ => println!("{}", sussify(input, &args, &mut rng)),
        },
        None => read_from_input(),
    }
}
