use crate::{
    config::CONFIG,
    database::{establish_connection, models::article},
    router::{md2html, TEMPLATES},
};
use hyper::{Body, Request, Response};
use matchit::Params;
use tera::Context;

pub async fn handle(_req: Request<Body>, params: Params<'_, '_>) -> Option<Response<Body>> {
    let year = params.get("year")?;
    let month = params.get("month")?;
    if year.len() != 4 || month.len() != 2 {
        return None;
    }
    let year = year.parse().ok()?;
    let month = month.parse().ok()?;
    if !(1..=12).contains(&month) {
        return None;
    }
    log::debug!("Request archive page: year = {}, month = {}", year, month);
    let mut articles = article::read_by_archive(&establish_connection(), year, month).ok()?;
    for atc in articles.iter_mut() {
        atc.content = md2html(&atc.content);
    }

    let cfg = CONFIG.get().unwrap();
    let mut content = Context::new();
    content.insert("site", &cfg.site);
    content.insert("title", &format!("Archive: {}-{}", year, month));
    content.insert("articles", &articles);

    let body = TEMPLATES
        .get()
        .unwrap()
        .render("list.html", &content)
        .unwrap();
    Some(Response::new(Body::from(body)))
}
