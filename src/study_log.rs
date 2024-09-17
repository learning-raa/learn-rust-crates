use anyhow::Result;

pub fn dive_in() -> anyhow::Result<()> {
    println!("!!!!");
    log::info!("314\n15 {}", 44);
    println!("!8888888!!!");

    Ok(())
}

//  //  //  //  //  //  //  //
use log::{Level, Metadata, Record};

static RAA_LOGGER: RaaLogger = RaaLogger;
struct RaaLogger;

impl log::Log for RaaLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("RAA: {}:{} - {}", record.level(), record.target(), record.args());
        }
    }

    fn flush(&self) {}
}




//  //  //  //  //  //  //  //
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_dive_in() -> Result<()> {
        log::set_logger(&RAA_LOGGER).unwrap();
        log::set_max_level(log::LevelFilter::Trace);
        dive_in()
    }
}
