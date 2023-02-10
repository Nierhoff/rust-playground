use kafka::producer::{Producer, Record};
use log::*;
use log4rs;

use consumer::MyConsumer;

mod consumer;

fn main() {
    log4rs::init_file("logging_config.yaml", Default::default()).unwrap();

    let broker = "localhost:9092";

    let hosts = vec![broker.to_owned()];

    let mut consumer = MyConsumer::new(hosts.clone(), "topic-in".to_owned());

    let mut producer = Producer::from_hosts(hosts.clone()).create().unwrap();

    loop {
        for ms in consumer.consume_events().iter() {
            for m in ms.messages() {
                // If the consumer receives an event, this block is executed
                let v = MyConsumer::get_event_data(m);
                let r = &Record::from_value("topic-out", v.to_string());
                producer.send(r).unwrap();
                info!("{}", v.to_string());
            }

            consumer.consume_messageset(ms);
        }

        consumer.commit_consumed();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
