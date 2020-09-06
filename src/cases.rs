use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

use askama::Template;

// enum CaseState{
//     OPEN,
//     CLOSED
// }

#[derive(Template)]
#[template(path = "case.html")]
#[derive(Serialize, Deserialize)]
pub struct Case {
    pub _id: String,
    // state: CaseState ,
    pub events: Vec<Value>,
    // pub events: Vec<serde_json::Map<String, Value>>,
}

pub fn example_case() -> Result<Case> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "_id":"4dc232c5-213c-43a7-8177-80d15bb9a33c",
            "events": [
                {
                    "_id": "a70a9747-4f47-4e70-b578-bc4b808d994c",
                    "name": "John Smith",
                    "age": 34,
                    "external": true,
                    "phone": "078176767",
                    "notes": "Called about blah blah"
                },
                {
                    "_id": "a88f3f19-ef98-4419-accc-c5270d58cabb",
                    "invoice": ""
                }
            ]
        }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let c: Case = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("Case with id {} has events with ids {} and {}", c._id, c.events[0]["_id"], c.events[0]["_id"]);
    // println!("Case with id {} has events with ids {} and {}", c["_id"], c["events"][0]["_id"], c["events"][0]["_id"]);

    Ok(c)
}
