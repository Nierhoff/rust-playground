use log::*;
use log4rs;

fn main() {
    log4rs::init_file("logging_config.yaml", Default::default()).unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
