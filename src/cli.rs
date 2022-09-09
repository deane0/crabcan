use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Activate debug mode
    #[clap(short, long)]
    debug: bool,

    /// Command to execute inside the container
    #[clap(short, long)]
    command: String,

    /// User ID to create inside the container
    #[clap(short, long)]
    pub uid: u32,

    /// Directory to mount as root of the container
    #[clap(parse(from_os_str), short = 'm', long = "mount")]
    pub mount_dir: PathBuf
}

pub fn parse_args() -> Args {
    // TODO
    // If args.debug: Setup log at debug level
    // Else: Setup log at info level

    // TODO
    // Validate arguments

    Args::parse()
}
