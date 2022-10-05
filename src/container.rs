use crate::{cli::Args, config::ContainerOpts, errors::ErrorCode};

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

pub fn start(args: Args) -> Result<(), ErrorCode> {
    let mut container = Container::new(args)?;
    if let Err(e) = container.create() {
        container.clean_exit()?;
        log::error!("Error while creating container: {:?}", e);
        return Err(e);
    }

    log::debug!("Finished, cleaning & exit");
    container.clean_exit()
}
