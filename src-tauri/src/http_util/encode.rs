use bytes::Bytes;
use flate2::read::DeflateDecoder;
use flate2::read::DeflateEncoder;
use flate2::read::GzDecoder;
use flate2::read::GzEncoder;
use flate2::Compression;
use http::HeaderValue;
use std::io::prelude::*;

pub enum SupportedEncoding {
    Gzip,
    Deflate,
    Identity,
}

impl SupportedEncoding {
    pub fn from(header: Option<&HeaderValue>) -> Result<Self, String> {
        let v = {
            if let Some(hv) = header {
                hv.to_str().unwrap()
            } else {
                ""
            }
        };
        match v {
            "gzip" => Ok(SupportedEncoding::Gzip),
            "deflate" => Ok(SupportedEncoding::Deflate),
            "identity" => Ok(SupportedEncoding::Identity),
            "" => Ok(SupportedEncoding::Identity),
            _ => Err("not supported encoding".to_string()),
        }
    }

    pub fn list() -> Vec<SupportedEncoding> {
        vec![
            SupportedEncoding::Gzip,
            SupportedEncoding::Deflate,
            SupportedEncoding::Identity,
        ]
    }

    pub fn is_supported(v: &str) -> Result<(), String> {
        match v {
            "gzip" => Ok(()),
            "deflate" => Ok(()),
            "identity" => Ok(()),
            "" => Ok(()),
            _ => Err("not supported encoding".to_string()),
        }
    }

    pub fn decode(&self, original: Bytes) -> Bytes {
        match self {
            Self::Gzip => {
                let ob = original.as_ref();
                let mut gd = GzDecoder::new(ob);
                let mut v = Vec::<u8>::new();
                gd.read_to_end(&mut v).unwrap();
                Bytes::from(v)
            }
            Self::Deflate => {
                let ob = original.as_ref();
                let mut dd = DeflateDecoder::new(ob);
                let mut v = Vec::<u8>::new();
                dd.read_to_end(&mut v).unwrap();
                Bytes::from(v)
            }
            Self::Identity => original,
        }
    }

    pub fn encode(&self, encoded_bytes: Bytes) -> Bytes {
        match self {
            Self::Gzip => {
                let eb = encoded_bytes.as_ref();
                let mut ge = GzEncoder::new(eb, Compression::best());
                let mut v = Vec::new();
                ge.read_to_end(&mut v);
                Bytes::from(v)
            }
            Self::Deflate => {
                let eb = encoded_bytes.as_ref();
                let mut de = DeflateEncoder::new(eb, Compression::best());
                let mut v = Vec::new();
                de.read_to_end(&mut v);
                Bytes::from(v)
            }
            Self::Identity => encoded_bytes,
        }
    }
}
