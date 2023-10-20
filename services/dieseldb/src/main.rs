use log::*;
use log4rs;

use logging;

fn main() {
    logging::logsetup();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
