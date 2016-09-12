extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
    let mut router = Router::new();

    router.get("/gifs/:id", get_gif, "id");
    fn get_gif(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Sorry, there is no gif.")))
    }

    Iron::new(router).http("localhost:3000").unwrap();
}
