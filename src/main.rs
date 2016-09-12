#![cfg_attr(all(feature="serde_type"), feature(custom_derive, plugin))]
#![cfg_attr(all(feature="serde_type"), plugin(serde_macros))]

extern crate rustc_serialize;
extern crate staticfile;
extern crate mount;
// extern crate handlebars;

use rustc_serialize::json::{Json, ToJson};
use std::collections::BTreeMap;

extern crate iron;
extern crate router;
// extern crate env_logger;
extern crate handlebars_iron as hbs;
#[cfg(not(feature = "serde_type"))]
// extern crate rustc_serialize;
#[cfg(feature = "serde_type")]
extern crate serde;
#[cfg(feature = "serde_type")]
extern crate serde_json;
#[macro_use]
extern crate maplit;

use std::error::Error;

use iron::prelude::*;
use iron::status;
use router::Router;
use hbs::{Template, HandlebarsEngine, DirectorySource, MemorySource};

use std::path::Path;
use staticfile::Static;
use mount::Mount;

#[cfg(not(feature = "serde_type"))]
// mod data {
//     use serde_json::value::{self, Value};
//     use std::collections::BTreeMap;
//
//     #[derive(Serialize, Debug)]
//     pub struct Gif {
//         title: String,
//         url: String,
//     }
//
//     pub fn make_data() -> BTreeMap<String, Value> {
//         let mut data = BTreeMap::new();
//
//
//         let gifs = vec![Gif {
//                              title: "gif1".to_string(),
//                              url: "https://media.giphy.com/media/9fbYYzdf6BbQA/giphy.gif".to_string(),
//                          },
//                          ];
//
//         data.insert("gifs".to_string(), value::to_value(&gifs));
//         // data.insert("engine".to_string(), value::to_value(&"serde_json"));
//         data
//     }
//
//     fn get_gif(_: &mut Request) -> IronResult<Response> {
//         let mut resp = Response::new();
//         let data = make_data();
//         resp.set_mut(Template::new("index", data)).set_mut(status::Ok);
//         Ok(resp)
//     }
// }

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
    // use data::*;
    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("src/templates/", ".hbs")));
    if let Err(r) = hbse.reload() {
        // panic!("{}", r.description());
        panic!("hoge");
    }

    let mut router = Router::new();
    router.get("/gifs/:id", get_gif, "id");
    // .get("/gifs/gif",
    // Static::new(Path::new("src/templates/gif/giphy.gif")),
    // "hoge");
    fn get_gif(_: &mut Request) -> IronResult<Response> {
        let mut resp = Response::new();
        let data = Giff {
            title: "gif11".to_string(),
            // url: "https://media.giphy.com/media/9fbYYzdf6BbQA/giphy.gif".to_string(), #<{(| url: "src/templates/gifs/giphy.gif".to_string(), |)}>#
            url: "gif/giphy.gif".to_string(), // url: "src/templates/gifs/giphy.gif".to_string(),
        };
        resp.set_mut(Template::new("index", data)).set_mut(status::Ok);
        // resp.set_mut(Template::with("<img src =\"{{url}}\">", data)).set_mut(status::Ok);
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
