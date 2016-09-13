extern crate multipart;
extern crate rustc_serialize;

extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;
extern crate handlebars_iron as hbs;

use std::fs::File;
use std::io::Read;
use multipart::server::{Multipart, Entries, SaveResult};

use rustc_serialize::json::{Json, ToJson};
use std::collections::BTreeMap;
use std::path::Path;

use iron::prelude::*;
use iron::status;
use router::Router;
use staticfile::Static;
use mount::Mount;
use hbs::{Template, HandlebarsEngine, DirectorySource};

struct Giff {
    title: String,
    url: String,
}

impl ToJson for Giff {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("title".to_string(), self.title.to_json());
        m.insert("url".to_string(), self.url.to_json());
        m.to_json()
    }
}


fn main() {
    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("src/templates/", ".hbs")));
    if let Err(r) = hbse.reload() {
        panic!("hoge");
    }

    let mut router = Router::new();
    router.get("/gifs/:id", get_gif, "id");
    fn get_gif(_: &mut Request) -> IronResult<Response> {
        let mut resp = Response::new();
        let data = Giff {
            title: "gif11".to_string(),
            url: "gif/giphy.gif".to_string(),
        };
        resp.set_mut(Template::new("index", data)).set_mut(status::Ok);
        println!{"{:?}",resp};
        Ok(resp)
    }

    let mut chain = Chain::new(router);
    chain.link_after(hbse);

    let mut mounts = Mount::new();
    mounts.mount("/", chain).mount("gifs/gif/", Static::new(Path::new("src/templates/gif/")));

    // Iron::new(router).http("localhost:3000").unwrap();
    // Iron::new(chain).http("localhost:3000").unwrap();
    Iron::new(mounts).http("localhost:3000").unwrap();
}

fn recieve_gif(request: &mut Request) -> IronResult<Response> {
    match Multipart::from_request(request) {
        Ok(mut multipart) => {
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

        println!(r#"Field "{}" is file "{}":"#, name, filename);
    }
    Ok(Response::with((status::Ok, "Multipart data is processed")))
}
