use clap::{error as clap_error, CommandFactory, Parser};
use rand::Rng;

// A very sus char
const SUS_CHAR: char = 'ඞ';

// Upper limit for the sussiness parameter
const DEFAULT_MAX_SUSSINESS: u32 = 10;
// How ඞ sus ඞ your string is by default
const DEFAULT_SUSSINESS: u32 = 1;

#[derive(Parser)]
struct Args {
    /// Input string to ඞ sussify ඞ
    #[arg(value_name = "string")]
    input: String,

    /// How ඞ sus ඞ you want the output string to be
    #[arg(short, long = "sus", default_value_t = DEFAULT_SUSSINESS, value_name = "sussiness")]
    sussiness: u32,

    /// Set a new upper limit for how ඞ sus ඞ the output string can be
    #[arg(long = "max-sus", default_value_t = DEFAULT_MAX_SUSSINESS, value_name = "max-sussiness")]
    max_sussiness: u32,
}

fn main() {
    // Parse arguments
    let args = Args::parse();

    // Retrieve a lazily-initialized thread-local RNG
    let mut rng = rand::thread_rng();

    if args.sussiness > args.max_sussiness {
        let mut cmd = Args::command();
        cmd.error(
            clap_error::ErrorKind::ValueValidation,
            format!(
                "Provided sussiness value {} is out of range (max {})",
                args.sussiness, args.max_sussiness
            ),
        )
        .exit()
    }

    // Loop through each character on input string
    // and replace them with SUS_CHAR, according to the sussiness parameter
    // Note: special characters, such as carriage return (CR) won't be replaced
    let output_string: String = args
        .input
        .chars()
        .map(|char| {
            if rng.gen_ratio(args.sussiness, args.max_sussiness) && !char.is_control() {
                SUS_CHAR
            } else {
                char
            }
        })
        .collect();

    // Print processed string
    println!("{}", output_string);
}
