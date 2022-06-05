To create the key.pem and cert.pem use the command. Fill in your own subject

$ openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem \
  -days 365 -sha256 -subj "/C=CN/ST=Fujian/L=Xiamen/O=TVlinux/OU=Org/CN=muro.lxd"
To remove the password, then copy nopass.pem to key.pem

$ openssl rsa -in key.pem -out nopass.pem


Getting Started
Clone this repository.
Edit Cargo.toml to configure your project name, as well as any other settings you need.
Ensure you have Postgresql installed.
Install sqlx-cli, with:
cargo install sqlx-cli --no-default-features --features postgres
Edit .env.example to use your settings.
Run the account migrations with sqlx migrate run.
Run the server:
cargo run

# Optionally, if you use cargo-watch:
cargo-watch -i templates -i static -i migrations -x run
If you're ready to push a release build, you probably want to run:

cargo build --release --no-default-features --features production
For configuring email dispatch, see the README in email_templates.

Accounts
Accounts is modeled to provide the most common features you would expect from a user system. It provides the following:

Registration
On signup, will dispatch an email for account verification.
Login
Password reset functionality is also provided.
Both password reset and account verification implement a one-time-use URL pattern. The flow is a mirror of what Django does; the URL is hashed based on account information, so once the password changes, the URL becomes invalid.

Registration and Login, by default, try to not leak that an existing user account might exist. If a user attempts to register with an already registered email address, the following will happen:

The person attempting to register will be shown the "normal" flow, as if they successfully signed up, being told to check their email to verify.
The already registered user is sent an email notifying that this happened, and includes a link to password reset - e.g, maybe they're a confused user who just needs to get back in.
Templates
Templates are written in Tera. If you've written templates in Django or Jinja2, they should be very familiar.

The provided templates has a top-level layout.html, which should be your global public layout. The templates/dashboard folder is what a user sees upon logging in.

In development, your templates are automatically reloaded on edit. App also provides a stock "an error happened" view, similar to what Django does.

In production, both of these are disabled.

Your template may use any of the environment variable starting with APP_.

Static
The static folder is where you can place any static things. In development, actix-files is preconfigured to serve content from that directory, in order to make life easier for just running on your machine. This is disabled in the production build, mostly because we tend to shove this behind Nginx. You can swap this as needed.

Forms
Writing the same email/password/etc verification logic is a chore, and one of the nicer things Django has is Form helpers for this type of thing. If you miss that, App has a forms-ish module that you can use.

For instance, you could do:

forms.rs

use serde::{Deserialize, Serialize};
use app::forms::{EmailField, PasswordField, Validation};

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct LoginForm {
    pub email: EmailField,
    pub password: PasswordField
}

impl Validation for LoginForm {
    fn is_valid(&mut self) -> bool {
        self.email.is_valid() && !self.password.value.is_empty()
    }
}
views.rs

/// POST-handler for logging in.
pub async fn authenticate(
    request: HttpRequest,
    form: Form<LoginForm>
) -> Result<HttpResponse> {
    if request.is_authenticated()? {
        return request.redirect("/dashboard/");
    }

    let mut form = form.into_inner();
    if !form.is_valid() {
        return request.render(400, "accounts/login.html", {
            let mut context = Context::new();
            context.insert("error", "Invalid email or password.");
            context.insert("form", &form);
            context
        });
    }
In this case, EmailField will check that the email is a mostly-valid email address. PasswordField will check that it's a "secure" password. Each Field type has an internal errors stack, so you can pass it back to your view and render errors as necessary.

For more supported field types, see the app/forms module.

Request Helpers
A personal pet peeve: the default actix-web view definitions are mind-numbingly verbose. Code is read far more than it's written, and thus App includes some choices to make writing views less of a headache: namely, access to things like database pools and authentication are implemented as traits on HttpRequest.

This makes the necessary view imports a bit cleaner, requiring just the prelude for some traits, and makes view definitons much cleaner overall. It's important to note that if, for whatever reason, you need to use standard actix-web view definitions, you totally can - App doesn't restrict this, just provides a (we think) nicer alternative.

Checking a User
You can call request.is_authenticated()? to check if a User is authenticated. This does not incur a database hit, but simply checks against the signed cookie session value.

You can call request.user()? to get the User for a request. This does not incur a database hit, and just loads cached information from the signed cookie session. Users are, by default anonymous - and can be checked with is_anonymous.

If you want the full user Account object, you can call Account::get(user.id, &db_pool).await?, or write your own method.

You can restrict access to only authenticated users on a URL basis by using app::guards::Auth; example usage can be found in src/dashboard/mod.rs.

Rendering a Template
You can call request.render(http_code, template_path, model), where:

http_code is an HTTP response code.
template_path is a relative path to the template you want to load.
model is a tera::Context.
Why is http_code just passing a number?`, you might ask. It's personal preference, mostly: developers are intelligent enough to know what an HTTP response code is, and it's far less verbose to just pass the number - and simple enough to scan when you're trying to track down something related to it.

request.render() makes two things available to you by default:

user, which is the current User instance from the signed cookie session.
flash_messages, which are one-time messages that you can have on a view.
Returning a JSON response
You can call request.json(http_code, obj), where objc is an object that can be serialized to JSON.

Returning a Redirect
You can call request.redirect(path), where path is where you want the user to go.

Setting a Flash Message
You can call request.flash(title, message) to add a Flash message to the request. This is a one-time message, typically used for, say, confirming that something worked.

Getting a Database Pool
You can call request.db_pool()? to get a database pool instance. This can be passed to whatever you need to call for database work.

Queuing a Background Job
You can use accounts/jobs for a basis to create your own background jobs, and register them similar to how they're done in src/main.rs.

You can call request.queue(MyJob {...})? to dispatch a job in the background.

Email
Email may be sent with the help of different drivers:

postmark (enabled with feature app/email-postmark),
sendgrid (enabled with feature app/email-sendgrid),
smtp (enabled with feature app/email-smtp).
You can enable several or all features, in which case all selected drivers will be tried until one success or all fails.

The End
Hopefully, this helps people get going with more web services in Rust, and provides a common base to work off of. There are three things to note here:

This is released under a "do-whatever-you-want" license. Just give credit if you use it as the basis for a web framework of your own.
Someone else is more than welcome to take this further and make a true web framework.
I would argue that actix-web, Rocket, and so on should really just have this kind of thing by default.
Thanks to every developer of a sub-crate used in this project; there are too many to list, but the Rust community is hands down one of the best out there.