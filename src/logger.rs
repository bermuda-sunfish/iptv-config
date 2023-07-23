use clap::App;
use anyhow::Result;
use log::debug;

/// Initializes the application logger. By default, the logger is initialized from
/// the environment (which outputs timestamps, log levels, and other logging details).
/// However, if the "simple text" args has been provided, the logger should be configured
/// to suppress supplementary logging information and only output logging messages.
pub fn init(app: &App<'static, 'static>) -> Result<()> {
    if app.clone().get_matches().is_present("text") {
        use env_logger::{Builder, Env};
        let env = Env::default();
        Builder::from_env(env)
            .format_level(false)
            .format_module_path(false)
            .format_timestamp(None)
            .init();
        debug!("The logger is initialized with a simple text message output");
    } else {
        // initialize with the default options
        env_logger::init();
        debug!("The logger is initialized with the default settings");
    }
    Ok(())
}