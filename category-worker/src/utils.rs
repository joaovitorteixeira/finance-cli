use rdkafka::ClientConfig;

pub fn get_config() -> ClientConfig {
    let mut kafka_config = ClientConfig::new();

    kafka_config.set("bootstrap.servers", "localhost:9092");
    //kafka_config.set("security.protocol", "SASL_SSL");
    //kafka_config.set("sasl.mechanism", "PLAIN");
    //kafka_config.set("username", "");
    //kafka_config.set("password", "");

    kafka_config
}
