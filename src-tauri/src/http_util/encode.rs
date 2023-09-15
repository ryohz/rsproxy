use bytes::Bytes;
use flate2::read::DeflateDecoder;
use flate2::read::DeflateEncoder;
use flate2::read::GzDecoder;
use flate2::read::GzEncoder;
use flate2::Compression;
use http::HeaderValue;
use std::io::prelude::*;

use super::error::HttpUtilError;

pub enum SupportedEncoding {
    Gzip,
    Deflate,
    Identity,
}

impl SupportedEncoding {
    pub fn from(header: Option<&HeaderValue>) -> Result<Self, HttpUtilError> {
        let r = {
            if let Some(hv) = header {
                hv.to_str()
            } else {
                Ok("")
            }
        };
        match r {
            Ok(v) => match v {
                "gzip" => Ok(SupportedEncoding::Gzip),
                "deflate" => Ok(SupportedEncoding::Deflate),
                "identity" => Ok(SupportedEncoding::Identity),
                "" => Ok(SupportedEncoding::Identity),
                _ => {
                    panic!("encoding check is not working");
                }
            },
            Err(e) => Err(HttpUtilError::MakeSupportedEncodingError(e.to_string())),
        }
    }

    pub fn list() -> Vec<SupportedEncoding> {
        vec![
            SupportedEncoding::Gzip,
            SupportedEncoding::Deflate,
            SupportedEncoding::Identity,
        ]
    }

    pub fn is_supported(v: &str) -> Result<(), HttpUtilError> {
        match v {
            "gzip" => Ok(()),
            "deflate" => Ok(()),
            "identity" => Ok(()),
            "" => Ok(()),
            _ => Err(HttpUtilError::UnsupportedEncodingError(format!(
                "content-encoding {} is not supported",
                v
            ))),
        }
    }

    pub fn decode(&self, original: Bytes) -> Result<Bytes, HttpUtilError> {
        match self {
            Self::Gzip => {
                let ob = original.as_ref();
                let mut gd = GzDecoder::new(ob);
                let mut v = Vec::<u8>::new();
                match gd.read_to_end(&mut v) {
                    Ok(_) => Ok(Bytes::from(v)),
                    Err(e) => Err(HttpUtilError::DecodeError(e.to_string())),
                }
            }
            Self::Deflate => {
                let ob = original.as_ref();
                let mut dd = DeflateDecoder::new(ob);
                let mut v = Vec::<u8>::new();
                match dd.read_to_end(&mut v) {
                    Ok(_) => Ok(Bytes::from(v)),
                    Err(e) => Err(HttpUtilError::DecodeError(e.to_string())),
                }
            }
            Self::Identity => Ok(original),
        }
    }

    pub fn encode(&self, encoded_bytes: Bytes) -> Result<Bytes, HttpUtilError> {
        match self {
            Self::Gzip => {
                let eb = encoded_bytes.as_ref();
                let mut ge = GzEncoder::new(eb, Compression::best());
                let mut v = Vec::new();
                match ge.read_to_end(&mut v) {
                    Ok(_) => Ok(Bytes::from(v)),
                    Err(e) => Err(HttpUtilError::EncodeError(e.to_string())),
                }
            }
            Self::Deflate => {
                let eb = encoded_bytes.as_ref();
                let mut de = DeflateEncoder::new(eb, Compression::best());
                let mut v = Vec::new();
                match de.read_to_end(&mut v) {
                    Ok(_) => Ok(Bytes::from(v)),
                    Err(e) => Err(HttpUtilError::EncodeError(e.to_string())),
                }
            }
            Self::Identity => Ok(encoded_bytes),
        }
    }
}