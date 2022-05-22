use app::tera::Context;
use std::pin::Pin;
use std::future::Future;
use std::env::var;

use app::serde::{Deserialize, Serialize};
use app::anyhow::{anyhow, Error};
use app::email::Email;
use app::jobs::{DEFAULT_QUEUE, Job, JobState};

use crate::web::accounts::Account;

/// A job for sending a Welcome email, generally dispatched after an account
/// has been verified.
#[derive(Debug, Serialize, Deserialize)]
pub struct SendWelcomeAccountEmail {
    pub to: i32
}

pub fn build_context(name: &str) -> Context {
    let mut context = Context::new();
    context.insert("name", name);
    context.insert(
        "help_url",
        &var("APP_HELP_URL").expect("APP_HELP_URL not set?"),
    );
    context
}

impl Job for SendWelcomeAccountEmail {
    type State = JobState;
    type Future = Pin<Box<dyn Future<Output=Result<(), Error>> + Send>>;

    const NAME: &'static str = "SendWelcomeAccountEmailJob";
    const QUEUE: &'static str = DEFAULT_QUEUE;

    fn run(self, state: JobState) -> Self::Future {
        Box::pin(async move {
            let (name, email) = Account::fetch_email(self.to, &state.pool)
                .await
                .map_err(|e| anyhow!("Error fetching user name/email: {:?}", e))?;

            let email = Email::new(
                "email/welcome",
                &[email],
                "Welcome to the service",
                build_context(&name),
                state.templates,
            );

            email?.send()?;
            
            Ok(())
        })
    }
}
