use crate::http_util::error::HttpUtilError;

pub async fn copy_body(body: hyper::Body) -> Result<(hyper::Body, hyper::Body), HttpUtilError> {
    let r = hyper::body::to_bytes(body).await;
    match r {
        Ok(b) => {
            let cb1 = b.clone();
            let cb2 = b.clone();
            Ok((hyper::Body::from(cb1), hyper::Body::from(cb2)))
        }
        Err(e) => Err(HttpUtilError::BodyCopyError(e.to_string())),
    }
}
