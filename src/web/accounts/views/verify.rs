use app::prelude::*;
use app::accounts::User;
use app::actix_web::{HttpRequest, web::Path};
use app::request::DatabasePool;
use app::Result;

use crate::web::accounts::Account;
use crate::web::accounts::views::utils::validate_token;

use crate::web::accounts::jobs::{SendWelcomeAccountEmail};

/// Just renders a standard "Check your email and verify" page.
pub async fn verify(request: HttpRequest) -> Result<HttpResponse> {
    request.render(200, "accounts/verify/index.html", Context::new())
}

/// Given a link (of form {uidb64}-{ts}-{token}), verifies the
/// token and user, signs them in, and redirects to the dashboard.
///
/// In general, we do not want to leak information, so any errors here
/// should simply report as "invalid or expired". 
pub async fn with_token(
    request: HttpRequest,
    Path((uidb64, ts, token)): Path<(String, String, String)>
) -> Result<HttpResponse> {
    if let Ok(account) = validate_token(&request, &uidb64, &ts, &token).await {
        let db = request.db_pool()?;
        Account::mark_verified(account.id, db).await?;

        request.set_user(User {
            id: account.id,
            name: account.name,
            is_admin: account.is_admin,
            is_anonymous: false
        })?;

        request.queue(SendWelcomeAccountEmail {
            to: account.id
        })?;

        return request.redirect("/dashboard/");
    }

    return request.render(200, "accounts/invalid_token.html", Context::new());
}