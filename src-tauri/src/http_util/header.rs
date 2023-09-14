use async_trait::async_trait;
use http::header::ACCEPT_ENCODING;
use hyper::{
    header::{HeaderName, HeaderValue},
    HeaderMap,
};
use serde_json::{Map, Value};
use std::str::FromStr;

#[async_trait]
impl crate::http_util::traits::Json for HeaderMap {
    async fn from_json(json_data: String) -> Self {
        let json_headers = json_data;
        let headers_map_from_json: Map<String, Value> =
            serde_json::from_str(&json_headers).unwrap();
        let mut headers = HeaderMap::new();
        for (name, value) in headers_map_from_json {
            let header_name = HeaderName::from_str(name.as_str()).unwrap();
            let header_value = HeaderValue::from_str(value.as_str().unwrap()).unwrap();
            headers.append(header_name, header_value);
        }
        headers
    }

    async fn to_json(&self) -> String {
        let mut header_json_map = Map::<String, Value>::new();
        for (name, value) in self {
            let name = name.to_string();
            let value = value.to_str().unwrap().to_string();
            let value = Value::from(value);
            let _ = header_json_map.insert(name, value);
        }
        serde_json::to_string(&header_json_map).unwrap()
    }

    fn check_encoding(&self) -> Result<(), String> {
        match self.get(ACCEPT_ENCODING) {
            Some(ae) => {
                let ae_list: Vec<&str> = ae.to_str().unwrap().split(',').collect();
                for e_ae in ae_list {
                    let e_ae = e_ae.replace(' ', "");
                    crate::http_util::encode::SupportedEncoding::is_supported(e_ae.as_str())?;
                }
                Ok(())
            }
            None => Ok(()),
        }
    }
}

impl crate::http_util::traits::ToString for hyper::Version {
    fn to_string(&self) -> String {
        match *self {
            hyper::Version::HTTP_09 => "HTTP/0.9",
            hyper::Version::HTTP_10 => "HTTP/1.0",
            hyper::Version::HTTP_11 => "HTTP/1.1",
            hyper::Version::HTTP_2 => "HTTP/2.0",
            hyper::Version::HTTP_3 => "HTTP/3.0",
            _ => {
                panic!("Invalid HTTP Version");
            }
        }
        .to_string()
    }
}

impl crate::http_util::traits::FromStr for hyper::Version {
    fn from_str(s: &str) -> Self {
        match s {
            "HTTP/0.9" => hyper::Version::HTTP_09,
            "HTTP/1.0" => hyper::Version::HTTP_10,
            "HTTP/1.1" => hyper::Version::HTTP_11,
            "HTTP/2.0" => hyper::Version::HTTP_2,
            "HTTP/3.0" => hyper::Version::HTTP_3,
            _ => {
                panic!("Invalid HTTP Version");
            }
        }
    }
}
