use clap::Parser;
use rand::Rng;

// A very sus char
const SUS_CHAR: char = 'ඞ';

// Upper limit for the sussiness parameter
const MAX_SUSSINESS: u32 = 10;

// A custom parser for the sussiness parameter to do some bounds checking
fn sussinness_parser(sussiness_str: &str) -> Result<u32, String> {
    // Try to parse parameter to u32
    match sussiness_str.trim().parse() {
        Ok(sussiness) => {
            // Check if that u32 is in bounds
            if sussiness <= MAX_SUSSINESS {
                Ok(sussiness)
            } else {
                Err(format!(
                    "Number {} is out of range (max {})",
                    sussiness, MAX_SUSSINESS
                ))
            }
        }
        Err(_) => Err(String::from(
            "Couldn't parse string to unsigned integer (maybe you provided a negative number?)",
        )),
    }
}

#[derive(Parser)]
struct Args {
    /// Input string to ඞ sussify ඞ
    #[arg(value_name = "string")]
    input: String,

    /// How ඞ sus ඞ you want the output string to be
    #[arg(
        short,
        long = "sus",
        default_value_t = 1,
        value_name = "sussiness",
        value_parser=sussinness_parser
    )]
    sussiness: u32,
}

fn main() {
    // Parse arguments
    let args = Args::parse();

    // Retrieve a lazily-initialized thread-local RNG
    let mut rng = rand::thread_rng();

    // Loop through each character on input string
    // and replace them with SUS_CHAR, according to the sussiness parameter
    // Note: special characters, such as carriage return (CR) won't be replaced
    let output_string: String = args
        .input
        .chars()
        .map(|char| {
            if rng.gen_ratio(args.sussiness, MAX_SUSSINESS) && !char.is_control() {
                SUS_CHAR
            } else {
                char
            }
        })
        .collect();

    // Print processed string
    println!("{}", output_string);
}
