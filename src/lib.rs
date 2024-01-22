use serde_json::Value;
use std::collections::HashMap;

pub struct Query {
    pub occurance: i32,
    pub filter_conditions: HashMap<String, Value>,
    pub group_keys: Vec<String>,
    pub time_frame: Option<i32>,
}

#[derive(Debug, PartialEq)]
pub struct Pattern {
    pub matched_count: i32,
}

fn get_filtered_events(
    events: &[HashMap<String, Value>],
    filter_conditions: HashMap<String, Value>,
) -> Vec<&HashMap<String, Value>> {
    let filtered_events: Vec<_> = events
        .iter()
        .filter(|event| {
            filter_conditions
                .iter()
                .all(|(key, value)| value == &Value::Null || event.get(key) == Some(value))
        })
        .collect();

    filtered_events
}

pub fn find_pattern(events: &[HashMap<String, Value>], query: Query) -> Pattern {
    // get filtered events
    let filtered_events = get_filtered_events(events, query.filter_conditions);

    // match pattern
    // if no time_frame
    let mut grouped_results: HashMap<String, Vec<&HashMap<String, _>>> = HashMap::new();
    for event in filtered_events.iter() {
        let key_combination = query.group_keys
            .iter()
            .filter_map(|key| event.get(key).map(|value| value.as_str().unwrap()))
            .collect::<Vec<&str>>()
            .join("|#|").to_string();


        grouped_results.entry(key_combination).or_default().push(event);
    }

    let mut all_matched_set = vec![];
    let mut final_result = HashMap::new();
    for (result_key, result_value) in grouped_results.iter() {
        if result_value.len() as i32 >= query.occurance {
            let chunked_result_value: Vec<Vec<_>> = result_value
                .chunks(query.occurance as usize)
                .filter_map(|chunk| Some(chunk.to_vec()).filter(|_| chunk.len() as i32 == query.occurance))
                .collect();
            final_result.insert(result_key, chunked_result_value.clone());
            all_matched_set.extend(chunked_result_value.clone())
        }
    }

    println!("{final_result:?} ++++++++++++++++++");

    Pattern {
        matched_count: all_matched_set.len() as i32,
    }
}
