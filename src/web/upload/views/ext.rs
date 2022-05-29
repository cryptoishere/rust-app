use app::prelude::*;
use app::Result;
// use log::{debug, error, log_enabled, info, Level};
// use serde::{Deserialize, Serialize};

use app::actix_web::{post, get, web, App, HttpServer, Result as OResult};
use serde::Deserialize;

use std::fs::{self, DirBuilder};

#[derive(Deserialize)]
pub struct Info {
    pub username: String,
}

pub async fn post_json(info: web::Json<Info>) -> OResult<String> {
    Ok(format!("Welcome {}!", info.username))
}

pub async fn post_bytes(body: web::Bytes) -> String {
    let bin = body;
    fs::write("some-file\\foo.docx", bin).unwrap();

    format!("Body {:?}!", b"saved")

}

pub async fn post_form(form: web::Form<Info>) -> String {
    format!("Welcome {}!", form.username)
}

pub async fn main(_request: HttpRequest) -> Result<HttpResponse> {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form action="/upload/json/" method="post" enctype="application/x-www-form-urlencoded">
                <input type="text" name="username"/>
                <button type="submit">Submit</button>
            </form>
            <form action="/upload/bytes/" method="post">
                <input type="text" name="bytes"/>
                <button type="submit">Submit</button>
            </form>
            <form action="/upload/bytes/" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <button type="submit">Submit</button>
            </form>
            <form action="/upload/form/" method="post" enctype="application/x-www-form-urlencoded">
                <input type="text" name="username"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;

    Ok(HttpResponse::Ok().body(html))
}