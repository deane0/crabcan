use crate::{cli::Args, config::ContainerOpts, errors::ErrorCode};

use nix::sys::utsname::uname;
use scan_fmt::scan_fmt;

pub struct Container {
    config: ContainerOpts,
}

impl Container {
    pub fn new(args: Args) -> Result<Container, ErrorCode> {
        let config = ContainerOpts::new(args.command, args.uid, args.mount_dir)?;

        Ok(Container { config })
    }

    pub fn create(&mut self) -> Result<(), ErrorCode> {
        log::debug!("Creation finished");
        Ok(())
    }

    pub fn clean_exit(&mut self) -> Result<(), ErrorCode> {
        log::debug!("Cleaning container");
        Ok(())
    }
}

pub const MINIMAL_KERNEL_VERSION: f32 = 4.8;

pub fn check_linux_version() -> Result<(), ErrorCode> {
    let host = uname().unwrap(); // TODO
    let release = host.release().to_str().unwrap(); // TODO
    log::debug!("Linux release: {}", release);

    if let Ok(version) = scan_fmt!(release, "{f}.{}", f32) {
        if version < MINIMAL_KERNEL_VERSION {
            return Err(ErrorCode::NotSupported(0));
        }
    } else {
        return Err(ErrorCode::ContainerError(0));
    }

    if host.machine() != "x86_64" {
        return Err(ErrorCode::NotSupported(1));
    }

    Ok(())
}

pub fn start(args: Args) -> Result<(), ErrorCode> {
    check_linux_version()?;
    let mut container = Container::new(args)?;
    if let Err(e) = container.create() {
        container.clean_exit()?;
        log::error!("Error while creating container: {:?}", e);
        return Err(e);
    }

    log::debug!("Finished, cleaning & exit");
    container.clean_exit()
}
