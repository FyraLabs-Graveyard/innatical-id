extern crate directories;
extern crate shared;

use clap::{App, arg};
use shared::config::get_config_directory;

fn app() -> Result<(), &'static str> {
    let matches = App::new("Innatical ID")
        .version("1.0") // TODO use Cargo env variables
        .author("Innatical LLC <contact@innatical.com>")
        .arg(arg!(-c --config <CONFIG>).required(false))
        .get_matches();

    println!("{}", get_config_directory(matches.value_of("config")).unwrap());
    
    Ok(())
}

fn main() {
    std::process::exit(match app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}
