use env_logger::Builder;
use log::LevelFilter;

pub fn init() {
    let mut builder = Builder::from_default_env();

    builder
        .format_level(true)
        .format_timestamp_millis()
        .format_target(true)
        .filter(None, LevelFilter::Info)
        .init();

}
