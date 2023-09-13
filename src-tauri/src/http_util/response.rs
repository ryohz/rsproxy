use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub headers: String,
    pub body: String,
    pub url: String,
    pub status: u16,
    pub version: String,
    pub piloted: bool,
}

impl Response {
    pub fn new() -> Self {
        Response {
            headers: "".to_string(),
            body: "".to_string(),
            url: "".to_string(),
            version: "".to_string(),
            status: 500,
            piloted: false,
        }
    }
    
    
}
