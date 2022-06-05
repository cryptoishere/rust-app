use app::prelude::*;
use app::Result;
use app::csv;

use std::fs::{self, DirBuilder};
use std::{io};
// use app::error::Error;
use std::path::PathBuf;
use std::borrow::Cow;
// use std::fs::File;
use std::collections::HashMap;

use crate::web::accounts::jobs::{SendWelcomeAccountEmail};

use log::{debug, error, log_enabled, info, Level};

use serde::{Deserialize, Serialize};

use app::actix_extract_multipart::*;
// Accepted files extensions
const FILES_EXTENSIONS: [&str; 1] = [/*"image/png", "image/jpeg",*/ "text/csv"];

pub struct CSVForm {
    pub string_param: String,
    pub number_u_param: u32,
    pub file_param: Option<File>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    latitude: f64,
    longitude: f64,
    // #[serde(rename = "Population")]
    // When applied to Option fields, it will convert any deserialization error into a None value.
    #[serde(deserialize_with = "csv::invalid_option")]
    population: Option<u64>,
    city: String,
    state: String,
}


#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct WriteRecord<'a> {
    city: &'a str,
    state: &'a str,
    population: Option<u64>,
    latitude: f64,
    longitude: f64,
}

pub async fn upload(request: HttpRequest) -> Result<HttpResponse> {
    // let user = request.user()?;

    // request.render(200, "upload/index.html", {
    //     let mut context = Context::new();

    //     context
    // })

    Ok(HttpResponse::Ok().body(include_str!("..\\..\\..\\..\\templates\\upload\\upload.html")))
}

pub async fn csv(payload: Multipart<CSVForm>) -> HttpResponse {
    println!("Value of string_param: {}", &payload.string_param);
    println!("Value of number_u_param: {}", &payload.number_u_param);
    println!(
        "File: {}",
        if payload.file_param.is_some() {
            "YES"
        } else {
            "NO"
        }
    );

    if let Some(file) = &payload.file_param {
        // We getting a file, we can, for example, check file type, saving this file or do some other stuff
        if !FILES_EXTENSIONS.contains(&file.file_type().as_str()) {
            eprintln!("Wrong file format");
            return HttpResponse::BadRequest()
                .json(format!("File's extension must be: {:?}", FILES_EXTENSIONS));
        }

        if saving_file_function(file).is_err() {
            return HttpResponse::InternalServerError().json("");
        };
    }

    HttpResponse::Ok().json("Done")
}

fn saving_file_function(file: &File) -> std::result::Result<(), ()> {
    // Do some stuff here
    println!(
        "Saving file \"{}\" ({} bytes) successfully. Additional data: FileType: {}",
        file.name(),
        file.len(),
        file.file_type(),
    );

    println!("data: {:?}", file.data());

    Ok(())
}

pub async fn upload_test(request: HttpRequest) -> Result<HttpResponse> {
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
    
    run().await?;
    test_async().await?;
    run_1().await?;
    run_null().await?;
    // write_csv().await?;
    pipelining_csv().await?;

    request.render(200, "upload/index.html", {
        let mut context = Context::new();

        context
    })
}

async fn run() -> std::result::Result<(), Error> {
    // This way is that the type you use must match the order of fields as they appear in each record.
    // type Record = (String, String, Option<u64>, f64, f64);

    // type Record = HashMap<String, String>;

    // let file = File::open("some-file\\uspop.csv")?;
    // let mut rdr = csv::Reader::from_reader(file);
    // let mut rdr = csv::Reader::from_path("some-file\\uspop.csv")?;

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("some-file\\uspop.csv")?;

    // for result in rdr.records() {
    for result in rdr.deserialize() {
        let record: Record = result?;
        info!(target: "upload", "rdr.records {:?}", record);
    }

    Ok(())
}

async fn run_null() -> std::result::Result<(), Error> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("some-file\\uspop-null.csv")?;

    for result in rdr.deserialize() {
        let record: Record = result?;
        info!(target: "upload", "uspop-null.csv {:?}", record);
    }

    Ok(())
}

async fn run_1() -> std::result::Result<(), Error> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .double_quote(false)
        .escape(Some(b'\\'))
        .flexible(true)
        .comment(Some(b'#'))
        // .from_reader(io::stdin());
        .from_path("some-file\\wrestlers.csv")?;
    for result in rdr.records() {
        let record = result?;
        info!(target: "upload", "some-file\\wrestlers.csv {:?}", record);
    }
    Ok(())
}

async fn test_async() -> std::result::Result<(), Error> {
    let bytes_text = fs::read("some-file\\foo.txt")?;
    let foo_test: Cow<str> = String::from_utf8_lossy(&bytes_text);
    info!(target: "upload", "fs::read utf8 async {:?}", &foo_test);

    Ok(())
}

async fn write_csv() -> std::result::Result<(), Error> {
    // // let mut wtr = csv::Writer::from_writer(io::stdout());
    // let mut wtr = csv::Writer::from_path("some-file\\write.csv")?;
    // // Since we're writing records manually, we must explicitly write our
    // // header record. A header record is written the same way that other
    // // records are written.
    // wtr.write_record(&["City", "State", "Population", "Latitude", "Longitude"])?;
    // wtr.write_record(&["Davidsons Landing", "AK", "", "65.2419444", "-165.2716667"])?;
    // wtr.write_record(&["Kenai", "AK", "7610", "60.5544444", "-151.2583333"])?;
    // wtr.write_record(&["Oakman", "AL", "", "33.7133333", "-87.3886111"])?;

    // // A CSV writer maintains an internal buffer, so it's important
    // // to flush the buffer when you're done.
    // wtr.flush()?;
    // Ok(())


    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .quote_style(csv::QuoteStyle::NonNumeric)
        .from_path("some-file\\write.csv")?;

    wtr.write_record(&["City", "State", "Population", "Latitude", "Longitude"])?;
    wtr.write_record(&["Davidsons Landing", "AK", "", "65.2419444", "-165.2716667"])?;
    wtr.write_record(&["Kenai", "AK", "7610", "60.5544444", "-151.2583333"])?;
    wtr.write_record(&["Oakman", "AL", "", "33.7133333", "-87.3886111"])?;

    wtr.serialize(("Davidsons Landing", "AK", None::<u64>, 65.2419444, -165.2716667))?;
    wtr.serialize(("Kenai", "AK", Some(7610), 60.5544444, -151.2583333))?;
    wtr.serialize(("Oakman", "AL", None::<u64>, 33.7133333, -87.3886111))?;

    wtr.serialize(WriteRecord {
        city: "Davidsons Landing",
        state: "AK",
        population: None,
        latitude: 65.2419444,
        longitude: -165.2716667,
    })?;
    wtr.serialize(WriteRecord {
        city: "Kenai",
        state: "AK",
        population: Some(7610),
        latitude: 60.5544444,
        longitude: -151.2583333,
    })?;
    wtr.serialize(WriteRecord {
        city: "Oakman",
        state: "AL",
        population: None,
        latitude: 33.7133333,
        longitude: -87.3886111,
    })?;

    wtr.flush()?;
    Ok(())
}

async fn pipelining_csv() -> std::result::Result<(), Error> {
    // Get the query from the positional arguments.
    // If one doesn't exist, return an error.
    // let query = match env::args().nth(1) {
    //     None => return Err(From::from("expected 1 argument, but got none")),
    //     Some(query) => query,
    // };

    let query = Some(String::from("MA"));
    let query = query.unwrap();
    // let query = String::from("MA");

    // Build CSV readers and writers to stdin and stdout, respectively.
    let mut rdr = csv::Reader::from_path("some-file\\uspop.csv")?;
    let mut wtr = csv::Writer::from_path("some-file\\write.csv")?;

    // Before reading our data records, we should write the header record.
    wtr.write_record(rdr.headers()?)?;

    // Iterate over all the records in `rdr`, and write only records containing
    // `query` to `wtr`.
    for result in rdr.records() {
        let record = result?;
        if record.iter().any(|field| field == &query) {
            wtr.write_record(&record)?;
        }
    }

    // CSV writers use an internal buffer, so we should always flush when done.
    wtr.flush()?;
    Ok(())
}