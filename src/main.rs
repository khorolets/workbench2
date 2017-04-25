extern crate chrono;
extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate serde_json;

#[macro_use] extern crate tera;

extern crate hyper;

use std::path::Path;

use iron::prelude::*;
use iron::status;
use mount::Mount;
use router::Router;
use staticfile::Static;


mod filters;
mod utils;

fn main() {
    let version = "0.2.0";

    fn greeting(_: &mut Request) -> IronResult<Response> {
        let template = utils::render_template("index.html");
        let mut response = Response::with((status::Ok, template));

        // Setting ContentType
        utils::set_content_type(&mut response);
        Ok(response)
    }

    fn page(req: &mut Request) -> IronResult<Response> {
        let ref page = req.extensions.get::<Router>()
            .unwrap()
            .find("page")
            .unwrap_or("404");

        let template = utils::render_template(*page);

        // Setting ContentType
        let mut response = Response::with((status::Ok, template));
        utils::set_content_type(&mut response);
        Ok(response)
    }

    let mut router = Router::new();

    router.get("/", greeting, "index");
    router.get("/:page", page, "query");

    let mut mount = Mount::new();
    mount
        .mount("/", router)
        .mount("/static/", Static::new(Path::new("static")));

    println!("Workbench2 ver. {}", version);
    println!("Running server on http://127.0.0.1:3000/");
    println!("Press Ctrl+C to stop server");
    Iron::new(mount).http("127.0.0.1:3000").unwrap();
}
