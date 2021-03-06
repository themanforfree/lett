use crate::database::{
    establish_connection,
    models::comment::{self, NewComment},
};
use hyper::{Body, Request, Response};
use matchit::Params;
use serde::Deserialize;

#[derive(Deserialize)]
struct CommentParams {
    method: String,
    #[serde(default)]
    cid: u32,
}
pub async fn handle(req: Request<Body>, _params: Params<'_, '_>) -> Option<Response<Body>> {
    log::debug!("Post to Comment");
    let body = hyper::body::to_bytes(req.into_body()).await.ok()?;
    let params: CommentParams = serde_urlencoded::from_bytes(&body).unwrap();
    match params.method.as_str() {
        "new" => {
            let article = NewComment::from(body);
            match comment::create(&establish_connection(), &article) {
                Ok(u) => Some(Response::new(Body::from(format!(
                    "Crate {} comment success",
                    u
                )))),
                Err(_) => Some(Response::new(Body::from("Crate comment failed"))),
            }
        }
        "delete" => match comment::delete(&establish_connection(), params.cid) {
            Ok(_) => Some(Response::new(Body::from("Delete comment success"))),
            Err(_) => Some(Response::new(Body::from("Delete comment failed"))),
        },
        _ => None,
    }
}
