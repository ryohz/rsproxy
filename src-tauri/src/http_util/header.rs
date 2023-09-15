use async_trait::async_trait;
use http::header::ACCEPT_ENCODING;
use hyper::{
    header::{HeaderName, HeaderValue},
    HeaderMap,
};
use serde_json::{Map, Value};
use std::str::FromStr;

use super::error::HttpUtilError;

#[async_trait]
impl crate::http_util::traits::HeaderMapMethods for HeaderMap {
    async fn from_json(json_data: String) -> Result<HeaderMap, HttpUtilError> {
        let json_h = json_data;
        let r = serde_json::from_str(&json_h);
        match r {
            Ok(hashmap_h) => {
                let hashmap_h: Map<String, Value> = hashmap_h;
                let mut h = HeaderMap::new();
                for (k, v) in hashmap_h {
                    match HeaderName::from_str(k.as_str()) {
                        Ok(k) => match v.as_str() {
                            Some(v_str) => {
                                match HeaderValue::from_str(v_str) {
                                    Ok(v) => {
                                        h.append(k, v);
                                    }
                                    Err(e) => {
                                        return Err(HttpUtilError::JsonHeadersParseError(
                                            e.to_string(),
                                        ));
                                    }
                                };
                            }
                            None => {
                                return Err(HttpUtilError::JsonHeadersParseError(
                                    "header value is empty".to_string(),
                                ));
                            }
                        },
                        Err(e) => {
                            return Err(HttpUtilError::JsonHeadersParseError(e.to_string()));
                        }
                    }
                }
                Ok(h)
            }
            Err(e) => Err(HttpUtilError::JsonHeadersParseError(e.to_string())),
        }
    }

    async fn json(&self) -> Result<String, HttpUtilError> {
        let mut header_json_map = Map::<String, Value>::new();
        for (name, value) in self {
            let name = name.to_string();

            match value.to_str() {
                Ok(v_str) => {
                    let value = Value::from(v_str);
                    let _ = header_json_map.insert(name, value);
                }
                Err(e) => {
                    return Err(HttpUtilError::HeaderConvertError(e.to_string()));
                }
            }
        }

        match serde_json::to_string(&header_json_map) {
            Ok(h) => Ok(h),
            Err(e) => Err(HttpUtilError::HeaderConvertError(e.to_string())),
        }
    }

    fn check_encoding(&self) -> Result<(), HttpUtilError> {
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

impl crate::http_util::traits::VersionMethods for hyper::Version {
    fn to_string(&self) -> Result<String, HttpUtilError> {
        match *self {
            hyper::Version::HTTP_09 => Ok("HTTP/0.9".to_string()),
            hyper::Version::HTTP_10 => Ok("HTTP/1.0".to_string()),
            hyper::Version::HTTP_11 => Ok("HTTP/1.1".to_string()),
            hyper::Version::HTTP_2 => Ok("HTTP/2.0".to_string()),
            hyper::Version::HTTP_3 => Ok("HTTP/3.0".to_string()),
            _ => {
                panic!("implementation for new http version is not available yet");
            }
        }
    }

    fn from_str(s: &str) -> Result<hyper::Version, HttpUtilError> {
        match s {
            "HTTP/0.9" => Ok(hyper::Version::HTTP_09),
            "HTTP/1.0" => Ok(hyper::Version::HTTP_10),
            "HTTP/1.1" => Ok(hyper::Version::HTTP_11),
            "HTTP/2.0" => Ok(hyper::Version::HTTP_2),
            "HTTP/3.0" => Ok(hyper::Version::HTTP_3),
            _ => Err(HttpUtilError::InvalidHttpVersionError(format!(
                "{} is invalid version",
                s
            ))),
        }
    }
}
