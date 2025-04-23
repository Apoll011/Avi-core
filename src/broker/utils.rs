use crate::broker::EmbeddedBroker;
use rumqttd::{Config, ConnectionSettings, RouterConfig, ServerSettings};
use std::collections::HashMap;

fn get_config() -> Config {
    Config {
        id: 0,
        router: RouterConfig {
            max_connections: 100,
            max_outgoing_packet_count: 200,
            max_segment_size: 104_857_600,
            max_segment_count: 10,
            custom_segment: None,
            initialized_filters: None,
            shared_subscriptions_strategy: Default::default(),
        },
        v4: {
            let mut map = HashMap::new();
            map.insert(
                "v4-1".to_string(),
                ServerSettings {
                    name: "v4-1".to_string(),
                    listen: "0.0.0.0:1883".to_string().parse().unwrap(),
                    tls: None,
                    next_connection_delay_ms: 1,
                    connections: ConnectionSettings {
                        connection_timeout_ms: 60_000,
                        max_payload_size: 20_480,
                        max_inflight_count: 100,
                        auth: None,
                        external_auth: None,
                        dynamic_filters: true,
                    },
                },
            );
            Some(map)
        },
        v5: {
            let mut map = HashMap::new();
            map.insert(
                "v5-1".to_string(),
                ServerSettings {
                    name: "v5-1".to_string(),
                    listen: "0.0.0.0:1884".to_string().parse().unwrap(),
                    tls: None,
                    next_connection_delay_ms: 1,
                    connections: ConnectionSettings {
                        connection_timeout_ms: 60_000,
                        max_payload_size: 20_480,
                        max_inflight_count: 100,
                        auth: None,
                        external_auth: None,
                        dynamic_filters: true,
                    },
                },
            );
            Some(map)
        },
        ws: {
            let mut map = HashMap::new();
            map.insert(
                "ws-1".to_string(),
                ServerSettings {
                    name: "ws-1".to_string(),
                    listen: "0.0.0.0:8083".to_string().parse().unwrap(),
                    tls: None,
                    next_connection_delay_ms: 1,
                    connections: ConnectionSettings {
                        connection_timeout_ms: 60_000,
                        max_payload_size: 20_480,
                        max_inflight_count: 500,
                        auth: None,
                        external_auth: None,
                        dynamic_filters: true,
                    },
                },
            );
            Some(map)
        },
        prometheus: None,
        cluster: None,
        console: None,
        bridge: None,
        metrics: None,
    }
}

pub fn start_mqtt() -> EmbeddedBroker {
    let config = get_config();

    EmbeddedBroker::new(config)
}
