//! Upload views.

mod upload;
mod ext;
pub use upload::{upload};
pub use ext::{main, post_json, post_bytes, post_form};