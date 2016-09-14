extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate multipart;

use std::path::Path;
use std::fs::File;
use std::io::Read;

use iron::prelude::*;
use iron::status;
use staticfile::Static;
use mount::Mount;

use multipart::server::{Multipart, Entries, SaveResult};

fn main() {
    let mut mounts = Mount::new();
    mounts.mount("/gifs/", Static::new(Path::new("src/templates/gif/")))
        .mount("/", process_request);
    Iron::new(mounts).http("localhost:3000").unwrap();
}

fn process_request(request: &mut iron::prelude::Request) -> IronResult<Response> {
    // Getting a multipart reader wrapper
    match Multipart::from_request(request) {
        Ok(mut multipart) => {
            // Fetching all data and processing it.
            // save_all() reads the request fully, parsing all fields and saving all files
            // in a new temporary directory under the OS temporary directory.
            match multipart.save_all() {
                SaveResult::Full(entries) => process_entries(entries),
                SaveResult::Partial(entries, error) => {
                    try!(process_entries(entries));
                    Err(IronError::new(error, status::InternalServerError))
                }
                SaveResult::Error(error) => Err(IronError::new(error, status::InternalServerError)),
            }
        }
        Err(_) => Ok(Response::with((status::BadRequest, "The request is not multipart"))),
    }
}

/// Processes saved entries from multipart request.
/// Returns an OK response or an error.
fn process_entries(entries: Entries) -> IronResult<Response> {
    for (name, field) in entries.fields {
        println!(r#"Field "{}": "{}""#, name, field);
    }

    for (name, savedfile) in entries.files {
        let filename = match savedfile.filename {
            Some(s) => s,
            None => "None".into(),
        };
        let mut file = match File::open(savedfile.path) {
            Ok(file) => file,
            Err(error) => {
                return Err(IronError::new(error,
                                          (status::InternalServerError,
                                           "Server couldn't save file")))
            }
        };
        let mut contents = String::new();
        if let Err(error) = file.read_to_string(&mut contents) {
            return Err(IronError::new(error, (status::BadRequest, "The file was not a text")));
        }

        println!(r#"Field "{}" is file "{}":"#, name, filename);
        println!("{}", contents);
    }
    Ok(Response::with((status::Ok, "Multipart data is processed")))
}
