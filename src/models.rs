
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use askama::Template;

#[derive(Template)]
#[template(path = "case.html")]
#[derive(Serialize, Deserialize)]
#[derive(Queryable)]
pub struct Case {
    pub id: Uuid,
    pub data: Value,
}

#[derive(Queryable)]
pub struct Event {
    pub id: Uuid,
    pub case_id: Uuid,
    pub data: Value,
}
