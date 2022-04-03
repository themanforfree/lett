use super::{HyperReq, HyperRes};

pub async fn handle(req: HyperReq) -> HyperRes {
    let path = req.uri().path();
    HyperRes::new(hyper::Body::from(format!("Path: {} Not Found", path)))
}
