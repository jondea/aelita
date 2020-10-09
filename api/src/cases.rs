use askama::Template;
use crate::models::Case;

#[derive(Template)]
#[template(path = "cases.html")]
pub struct CasesTemplate {
    pub cases: Vec<Case>
}
