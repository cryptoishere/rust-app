use app::prelude::*;
use app::Result;

use std::fs::{self, DirBuilder};
use std::{io};
// use app::error::Error;
use std::path::PathBuf;
use std::borrow::Cow;

use crate::web::accounts::jobs::{SendWelcomeAccountEmail};

use log::{debug, error, log_enabled, info, Level};

/// Returns an overview of everything in the system.
pub async fn upload(request: HttpRequest) -> Result<HttpResponse> {
    // let user = request.user()?;

    // let path = "/tmp/foo/bar/baz";
    // DirBuilder::new()
    //     .recursive(true)
    //     .create(path).unwrap();

    // info!(target: "upload", "DirBuilder {:?}", fs::metadata(path));
    // assert!(fs::metadata(path).unwrap().is_dir());
    
    let path = fs::canonicalize(".\\test.sh").unwrap();

    info!(target: "upload", "canonicalize {:?}", path);

    fs::create_dir_all(".\\some-file").unwrap();

    for entry in fs::read_dir(".").unwrap() {
        let entry = entry.unwrap();
        info!(target: "upload", "read_dir entry {:?}", entry);
        let path = entry.path();
        info!(target: "upload", "read_dir path {:?}", path);
    }

    let mut path_collection: Vec<PathBuf> = Vec::new();

    let mut entries = fs::read_dir(".")?
        .map(|res| {
            res.map(|e| {
                path_collection.push(e.path());
                e.path()
            })
        });
        // .collect::<Result<Vec<_>, std::io::Error>>()?;

    for entry in entries {
        // let test = entry.unwrap().to_str().unwrap();
        info!(target: "upload", "read_dir entries -> entry {:?}", entry);
    }

    info!(target: "upload", "path_collection {:?}", path_collection);


    fs::write("some-file\\foo.txt", b"Lorem ipsum")?;
    let foo = fs::read("some-file\\foo.txt")?;
    info!(target: "upload", "fs::read {:?}", foo);
    
    info!(target: "upload", "fs::read utf8 {:?}", &String::from_utf8_lossy(&fs::read("some-file\\foo.txt")?));

    let bytes_text = fs::read("some-file\\foo.txt")?;
    let foo_test: Cow<str> = String::from_utf8_lossy(&bytes_text);
    info!(target: "upload", "fs::read utf8 {:?}", &foo_test);


    request.render(200, "upload/index.html", {
        let mut context = Context::new();

        context
    })
}