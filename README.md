## EVENT PATTERN FINDER (SIMPLE FINITE STATE MACHINE), Implemented in Rust Programming Language ##

### Query Struct
The Query struct is defined with the following fields:

- occurance: An integer specifying the occurrence requirement for pattern matching.
- filter_conditions: A HashMap with string keys and serde_json Value values representing filtering conditions.
- group_keys: A vector of string keys used for grouping events.
- time_frame: An optional integer specifying a time frame for the query.

