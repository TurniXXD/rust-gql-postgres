use serde_json::{json, Map, Value};

pub fn naming_pattern_to_json(name: &str) -> Value {
    let mut keys = name.split("/");
    let mut obj = Map::new();
    let mut current_obj = &mut obj;

    while let Some(key) = keys.next() {
        if keys.clone().count() > 0 {
            current_obj
                .entry(key.to_owned())
                .or_insert_with(|| Value::Object(Map::new()));
            current_obj = current_obj.get_mut(key).unwrap().as_object_mut().unwrap();
        } else {
            current_obj.entry(key.to_owned()).or_insert(Value::Null);
        }
    }

    json!(obj)
}
