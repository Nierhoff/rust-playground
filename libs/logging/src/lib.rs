use log::LevelFilter;
use log4rs;
use log4rs::config::Root;
use log4rs::Config;

pub fn logsetup() {
    let logfile = "logging_config.yaml";
    let b = std::path::Path::new(logfile).exists();
    if b {
        log4rs::init_file(logfile, Default::default()).unwrap();
    } else {
        let config = Config::builder()
            .build(Root::builder().build(LevelFilter::Info))
            .unwrap();
        log4rs::init_config(config).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::logsetup;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_user_ok() {
        logsetup();
    }
}
