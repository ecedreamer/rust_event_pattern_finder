use serde_json::Value;
use std::collections::HashMap;

use rust_event_pattern_finder::{find_pattern, Pattern, Query};

fn get_events() -> Vec<HashMap<String, Value>> {
    let events_str = r#"[
        {"timestamp": "2024-01-17T08:15:08", "event_type": "authentication", "user_id": "user123", "status": "Success", "ip_address": "192.168.1.100", "visited": "facebook.com"},
        {"timestamp": "2024-01-17T08:15:13", "event_type": "authentication", "user_id": "user233", "status": "Success", "ip_address": "192.168.1.101", "visited": "facebook.com"},
        {"timestamp": "2024-01-17T08:16:19", "event_type": "authentication", "user_id": "user233", "status": "failed", "ip_address": "192.168.1.100", "visited": "linkedin.com"},
        {"timestamp": "2024-01-17T08:15:20", "event_type": "authentication", "user_id": "user123", "status": "success", "ip_address": "192.168.1.102", "visited": "linkedin.com"},
        {"timestamp": "2024-01-17T08:15:24", "event_type": "authentication", "user_id": "user123", "status": "failed", "ip_address": "192.168.1.100", "visited": "linkedin.com"},
        {"timestamp": "2024-01-17T08:17:28", "event_type": "authentication", "user_id": "user123", "status": "success", "ip_address": "192.168.1.102", "visited": "linkedin.com"},
        {"timestamp": "2024-01-17T08:15:31", "event_type": "authentication", "user_id": "user233", "status": "failed", "ip_address": "192.168.1.101", "visited": "facebook.com"},
        {"timestamp": "2024-01-17T08:15:33", "event_type": "authentication", "user_id": "user123", "status": "failed", "ip_address": "192.168.1.102", "visited": "linkedin.com"},
        {"timestamp": "2024-01-17T08:20:37", "event_type": "authentication", "user_id": "user123", "status": "failed", "ip_address": "192.168.1.100", "visited": "linkedin.com"},
        {"timestamp": "2024-01-17T08:21:40", "event_type": "authentication", "user_id": "user123", "status": "success", "ip_address": "192.168.1.102", "visited": "facebook.com"}
    ]"#;

    let events: Vec<HashMap<String, Value>> =
        serde_json::from_str(events_str).expect("Failed to parse JSON");
    events
}

#[test]
fn test_find_pattern_matched() {
    let events = get_events();

    let mut condition = HashMap::new();
    condition.insert("status".to_owned(), Value::String("failed".to_owned()));
    let query = Query {
        occurance: 5,
        filter_conditions: condition.clone(),
        group_keys: vec![],
        time_frame: None,
    };
    let matched_pattern = find_pattern(&events, query);
    assert_eq!(matched_pattern, Pattern { matched_count: 1 });

    let query = Query {
        occurance: 2,
        filter_conditions: condition.clone(),
        group_keys: vec![],
        time_frame: None,
    };
    let matched_pattern = find_pattern(&events, query);
    assert_eq!(matched_pattern, Pattern { matched_count: 2 });
}

#[test]
fn test_find_pattern_unmatched() {
    let events = get_events();

    let mut condition = HashMap::new();
    condition.insert("status".to_owned(), Value::String("failed".to_owned()));
    let query = Query {
        occurance: 6,
        filter_conditions: condition.clone(),
        group_keys: vec![],
        time_frame: None,
    };
    let matched_pattern = find_pattern(&events, query);
    assert_eq!(matched_pattern, Pattern { matched_count: 0 });
}

#[test]
fn test_find_pattern_with_key_present() {
    let events = get_events();

    let mut condition = HashMap::new();
    condition.insert("status".to_owned(), Value::Null);
    let query = Query {
        occurance: 3,
        filter_conditions: condition.clone(),
        group_keys: vec![],
        time_frame: None,
    };
    let matched_pattern = find_pattern(&events, query);
    assert_eq!(matched_pattern, Pattern { matched_count: 3 });
}

#[test]
fn test_find_pattern_with_key_present_and_value_matched() {
    let events = get_events();

    let mut condition = HashMap::new();
    condition.insert("user_id".to_owned(), Value::String("user123".to_owned()));
    condition.insert("status".to_owned(), Value::Null);
    let query = Query {
        occurance: 3,
        filter_conditions: condition.clone(),
        group_keys: vec![],
        time_frame: None,
    };
    let matched_pattern = find_pattern(&events, query);
    assert_eq!(matched_pattern, Pattern { matched_count: 2 });
}

#[test]
fn test_find_pattern_with_single_group_key() {
    let events = get_events();

    let mut condition = HashMap::new();
    condition.insert("user_id".to_owned(), Value::String("user123".to_owned()));
    let group_keys = vec!["ip_address".to_owned()];
    let query = Query {
        occurance: 2,
        filter_conditions: condition.clone(),
        group_keys: group_keys,
        time_frame: None,
    };
    let matched_pattern = find_pattern(&events, query);
    assert_eq!(matched_pattern, Pattern { matched_count: 3 });
}

#[test]
fn test_find_pattern_with_multiple_group_key() {
    let events = get_events();

    let mut condition = HashMap::new();
    condition.insert("user_id".to_owned(), Value::String("user123".to_owned()));
    condition.insert("status".to_owned(), Value::Null);
    let group_keys = vec!["user_id".to_owned(), "visited".to_owned()];
    let query = Query {
        occurance: 2,
        filter_conditions: condition.clone(),
        group_keys: group_keys,
        time_frame: None,
    };
    let matched_pattern = find_pattern(&events, query);
    assert_eq!(matched_pattern, Pattern { matched_count: 3 });
}
