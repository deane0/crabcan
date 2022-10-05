use std::path::PathBuf;

use clap::Parser;

use crate::errors::ErrorCode;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Activate debug mode
    #[clap(short, long)]
    debug: bool,

    /// Command to execute inside the container
    #[clap(short, long)]
    pub command: String,

    /// User ID to create inside the container
    #[clap(short, long)]
    pub uid: u32,

    /// Directory to mount as root of the container
    #[clap(parse(from_os_str), short = 'm', long = "mount")]
    pub mount_dir: PathBuf,
}

pub fn parse_args() -> Result<Args, ErrorCode> {
    let args = Args::parse();

    if args.debug {
        setup_log(log::LevelFilter::Debug);
    } else {
        setup_log(log::LevelFilter::Info);
    }

    if !args.mount_dir.exists() || !args.mount_dir.is_dir() {
        return Err(ErrorCode::ArgumentInvalid("mount"));
    }

    Ok(args)
}

pub fn setup_log(level: log::LevelFilter) {
    env_logger::Builder::from_default_env()
        .format_timestamp_secs()
        .filter(None, level)
        .init();
}
