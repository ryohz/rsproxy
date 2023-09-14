use async_trait::async_trait;

pub trait ToString {
    fn to_string(&self) -> String;
}

pub trait FromStr {
    fn from_str(s: &str) -> Self;
}

#[async_trait]
pub trait Json {
    async fn from_json(json_data: String) -> Self;
    async fn to_json(&self) -> String;
    fn check_encoding(&self) -> Result<(), String>;
}
