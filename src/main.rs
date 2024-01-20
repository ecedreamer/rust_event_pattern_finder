use serde_json::Value;
use std::collections::HashMap;

use rust_event_pattern_finder::{find_pattern, Query};

fn get_events() -> Vec<HashMap<String, Value>> {
    let events_str = r#"[
        {"timestamp": "2024-01-17T08:15:08", "event_type": "authentication", "user_id": "user123", "status": "Success", "ip_address": "192.168.1.100", "visited": "linkedin.com"},
        {"timestamp": "2024-01-17T08:15:13", "event_type": "authentication", "user_id": "user123", "status": "Success", "ip_address": "192.168.1.101", "visited": "facebook.com"},
        {"timestamp": "2024-01-17T08:16:19", "event_type": "authentication", "user_id": "user123", "status": "failed", "ip_address": "192.168.1.100", "visited": "linkedin.com"},
        {"timestamp": "2024-01-17T08:15:20", "event_type": "authentication", "user_id": "user123", "status": "success", "ip_address": "192.168.1.102", "visited": "linkedin.com"},
        {"timestamp": "2024-01-17T08:15:24", "event_type": "authentication", "user_id": "user123", "status": "failed", "ip_address": "192.168.1.100", "visited": "facebook.com"},
        {"timestamp": "2024-01-17T08:17:28", "event_type": "authentication", "user_id": "user123", "status": "success", "ip_address": "192.168.1.102", "visited": "linkedin.com"},
        {"timestamp": "2024-01-17T08:15:31", "event_type": "authentication", "user_id": "user123", "status": "failed", "ip_address": "192.168.1.101", "visited": "facebook.com"},
        {"timestamp": "2024-01-17T08:15:33", "event_type": "authentication", "user_id": "user123", "status": "failed", "ip_address": "192.168.1.102", "visited": "linkedin.com"},
        {"timestamp": "2024-01-17T08:20:37", "event_type": "authentication", "user_id": "user123", "status": "failed", "ip_address": "192.168.1.100", "visited": "linkedin.com"},
        {"timestamp": "2024-01-17T08:21:40", "event_type": "authentication", "user_id": "user123", "status": "success", "ip_address": "192.168.1.102", "visited": "facebook.com"}
    ]"#;

    let events: Vec<HashMap<String, Value>> =
        serde_json::from_str(events_str).expect("Failed to parse JSON");
    events
}

fn main() {
    let events = get_events();
    let condition: HashMap<String, Value> = vec![
        ("user_id".to_owned(), Value::String("user123".to_owned())),
        ("status".to_owned(), Value::Null),
    ].into_iter().collect();
    let group_keys = vec!["visited".to_owned()];
    let query = Query {
        occurance: 2,
        filter_conditions: condition.clone(),
        group_keys: group_keys,
        time_frame: None,
    };
    let matched_pattern = find_pattern(&events, query);
    println!("{matched_pattern:?}");
}
