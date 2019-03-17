pub fn init(n: u64) {
    let log_level = match n {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };
    simplelog::TermLogger::init(log_level, simplelog::Config::default()).unwrap();

    debug!("verbosity: {:?}", log_level);
    if log_level >= log::LevelFilter::Debug {
        debug!("We are very verbose!");
    }
}
