use std::process::exit;

use errors::exit_with_return_code;

mod cli;
mod config;
mod container;
mod errors;

fn main() {
    match cli::parse_args() {
        Ok(args) => {
            log::info!("{:?}", args);
            exit_with_return_code(Ok(()));
        }
        Err(e) => {
            log::error!("Error while parsing arguments:\n\t{}", e);
            exit(e.get_return_code());
        }
    }
}
