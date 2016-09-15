extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate multipart;

use std::path::Path;
use std::fs::File;
use std::fs;
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
    // .mount("/", pr);
    Iron::new(mounts).http("localhost:3000").unwrap();
}

fn pr(request: &mut iron::prelude::Request) -> IronResult<Response> {
    println!{"request is {:?}", request};
    let mut body = String::new();
    let bodyy = &mut body;
    request.body.read_to_string(bodyy);
    println!{"header is{:}", request.headers};
    println!{"body is {:?}", bodyy};
    match Multipart::from_request(request) {
        Ok(mut multipart) => {
            multipart.foreach_entry(|entry| {
                // let body: String = String::new();
                // let data = entry.data;
                // data.read_to_string(body);
                // println!("body is: {:?}", body);
                // println!("body is: {:?}", entry.data.as_text());
                // println!("entry is: {}", entry.name);
            });
            // println!("boudary is :{:?}", multipart);
            // println!("name is :{}", multipart.read_entry().unwrap().unwrap().name);
            // println!("data is :{}",
            // multipart.read_entry().unwrap().unwrap().data.as_text().unwrap());
            // println!("data is :{}",
            //          multipart.read_entry()
            //              .unwrap()
            //              .unwrap()
            //              .data
            //              .as_file()
            //              .unwrap()
            //              .filename()
            //              .unwrap());
            Ok(Response::with((status::Ok, "Multipart data is processed")))
        }
        Err(_) => Ok(Response::with((status::BadRequest, "The request is not multipart"))),
    }
    // Ok(Response::with((status::BadRequest, "The request is not multipart")))
}
fn process_request(request: &mut iron::prelude::Request) -> IronResult<Response> {
    // Getting a multipart reader wrapper
    match Multipart::from_request(request) {
        Ok(mut multipart) => {
            println!("hogehogehoge");
            // Fetching all data and processing it.
            // save_all() reads the request fully, parsing all fields and saving all files
            // in a new temporary directory under the OS temporary directory.
            // match multipart.save_all() {
            match multipart.save_all_under(Path::new("./src/templates/")) {
                SaveResult::Full(entries) => {
                    println!("hoge");
                    // loop {}
                    process_entries(entries)
                }
                SaveResult::Partial(entries, error) => {
                    println!("partial");
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
    println!("process_entries");
    for (name, field) in entries.fields {
        println!(r#"Field "{}": "{}""#, name, field);
    }

    println!("files{:?}", entries.files);
    for (name, savedfile) in entries.files {
        println!("entries.files!!!");
        println!("savvevdddddddddddddd");
        let file_path = savedfile.path.clone();
        let filename = match savedfile.filename {
            Some(s) => s,
            None => "None".into(),
        };
        let mut file = match File::open(savedfile.path) {
            Ok(file) => {

                println!("savvevdddddddddddddd");
                file
            }
            Err(error) => {
                println!("not_savedddddddddddddddddddd");
                return Err(IronError::new(error,
                                          (status::InternalServerError,
                                           "Server couldn't save file")));
            }
        };
        // fs::copy(file_path, "./../gif/hoge.gif");
        // loop {}
        let file_name = filename.clone();
        fs::copy(file_path, format!("{}{}", "src/templates/gif/", file_name));
        println!("filenam is === {}", file_name);
        // fs::copy(file_path, "src/templates/gif/gif.gif");
        // fs::copy("/Users/200246/development/Rust/gifzo_rust_server/src/templates/multipart.\
        //           VB8HiH883msT/ZrDYnpBoA9tW",
        //          "src/templates/gif/gif.gif");
        // "/Users/200246/development/Rust/gifzo_rust_server/src/templates/gif/gif.gif");
        let mut contents = String::new();
        // if let Err(error) = file.read_to_string(&mut contents) {
        //     return Err(IronError::new(error, (status::BadRequest, "The file was not a text")));
        // }

        println!(r#"Field "{}" is file "{}":"#, name, filename);
        println!("{}", contents);
    }
    // loop {}
    Ok(Response::with((status::Ok, "Multipart data is processed2")))
}
