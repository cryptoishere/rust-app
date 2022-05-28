//! Admin upload.

use app::actix_web::web::{resource, scope, ServiceConfig};
use app::guards::Auth;

mod views;

pub fn configure(config: &mut ServiceConfig) {    
    let guard = Auth {
        redirect_to: "/accounts/login/"
    };

    config.service(scope("/upload/").wrap(guard)
        // Index
        .service(resource("").to(views::upload))
    );
}