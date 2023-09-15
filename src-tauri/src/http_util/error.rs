use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpUtilError {
    // ** body.rs
    #[error(" >>> failed to copy body >>> `{0}`")]
    BodyCopyError(String),
    // ** encode.rs
    #[error(" >>> failed to make SupportedEncoding onject >>> `{0}`")]
    MakeSupportedEncodingError(String),
    #[error(" >>> unsupported content-encoding >>> `{0}`")]
    UnsupportedEncodingError(String),
    #[error(" >>> failed to decode >>> `{0}`")]
    DecodeError(String),
    #[error(" >>> failed to encode >>> `{0}`")]
    EncodeError(String),
    // ** header.rs
    #[error(" >>> failed to parse json headers as hashmap >>> `{0}`")]
    JsonHeadersParseError(String),
    #[error(" >>> failed to convert headers to json >>> `{0}`")]
    HeaderConvertError(String),
    #[error(" >>> invalid http version >>> `{0}`")]
    InvalidHttpVersionError(String),
    // ** request.rs
    #[error(" >>> failed to create request from hyper::request >>> `{0}`")]
    RequestFromHyperError(String),
    #[error(" >>> failed to convert request to hyper::request >>> `{0}`")]
    RequestToHyperError(String),
    #[error(" >>> failed to send request to frontend >>> `{0}`")]
    RequestSendToFrontError(String),
    #[error(" >>> failed to receive modified request >>> `{0}`")]
    ModifiedRequestReceiveError(String),
    #[error(" >>> failed to copy request >>> `{0}`")]
    RequestCopyError(String),
    // ** response.rs
    #[error(" >>> failed to create response from hyper::response >>> `{0}`")]
    ResponseFromHyperError(String),
    #[error(" >>> failed to convert response to hyper::response >>> `{0}`")]
    ResponseToHyperError(String),
    #[error(" >>> failed to send response to frontend >>> `{0}`")]
    ResponseSendToFrontError(String),
    #[error(" >>> failed to receive modified response >>> `{0}`")]
    ModifiedResponseReceiveError(String),
    #[error(" >>> failed to copy response >>> `{0}`")]
    ResponseCopyError(String),
    #[error(" >>> failed to decode response >>> `{0}`")]
    ResponseDecodeError(String),
}
