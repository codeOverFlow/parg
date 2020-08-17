extern crate parg;

use parg::arg::Arg;
use parg::arg::Type;
use parg::cli_argument::CliArguments;
use parg::create_cli_argument;

fn main() {
    // Create required argument --config
    let a = Arg::with_value("config", Type::ReadAsString, true);
    // Create optional argument --thread
    let b = Arg::with_value("thread", Type::ReadAsU8, false);
    // Create optional argument with no value --verbose
    let c = Arg::without_value("verbose", false);

    // Create the cli
    let cli: CliArguments = create_cli_argument!(a, b, c);

    // parse args and get return status
    let return_status = cli.parse();
    if let Err(msg) = return_status {
        eprintln!("{}", msg);
        return;
    }
    println!("{}", cli);

    // get the value as a String
    let config: String = match cli.get_value("config") {
        Ok(value) => value,
        Err(msg) => {
            eprintln!("{}", msg);
            return;
        }
    };
    println!("config = {}", config);

    // get the value as a u8
    let thread: u8 = match cli.get_value("thread") {
        Ok(value) => value,
        Err(msg) => {
            eprintln!("{}", msg);
            return;
        }
    };
    println!("thread = {}", thread);

    // check if --verbose has been used
    let verbose = cli.exists("verbose");
    println!("verbose = {}", verbose);

    // get the value as anything other than u8 (its reading type) gives an error
    // The requested type for "thread" does not match the reading type !
    let thread: u16 = match cli.get_value("thread") {
        Ok(value) => value,
        Err(msg) => {
            eprintln!("{}", msg);
            return;
        }
    };
    println!("thread = {}", thread);
}
