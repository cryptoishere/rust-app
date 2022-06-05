//! Admin upload.

use app::actix_web::web::{self, resource, scope, ServiceConfig};
use app::guards::Auth;

mod views;

pub fn configure(config: &mut ServiceConfig) {    
    let guard = Auth {
        redirect_to: "/accounts/login/"
    };

    config.service(scope("/upload/").wrap(guard)
        //upload
        .service(resource("").to(views::upload))
        // .service(resource("/csv/").route(web::post().to(views::csv)))
        // /upload/extractors
        .service(resource("/ext").to(views::main))
        .service(resource("/json/").route(web::post().to(views::post_json)))
        .service(resource("/bytes/").route(web::post().to(views::post_bytes)))
        .service(resource("/form/").route(web::post().to(views::post_form)))
    );
}