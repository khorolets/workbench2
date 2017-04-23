extern crate chrono;
extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;

#[macro_use] extern crate tera;

extern crate hyper;

use std::path::Path;

use chrono::{UTC};
use iron::prelude::*;
use iron::status;
use mount::Mount;
use router::Router;
use staticfile::Static;

use tera::Context;

use hyper::header::{ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};

fn set_content_type(response: &mut Response) {
    //! Setting Content Type Header to the `response` before returning
    response.headers.set(
        ContentType(
            Mime(
                TopLevel::Text, SubLevel::Html, vec![]
            )
        )
    )
}

fn workbench_common_context() -> Context {
    //! Returns tera::Context that is common for every page
    let mut context = Context::new();
    context.add(
        "timestamp",
        &UTC::now().timestamp()
    );
    context
}

fn main() {

    fn greeting(_: &mut Request) -> IronResult<Response> {
        let tera = compile_templates!("templates/**/*");
        let context = workbench_common_context();
        let template = tera.render("index.html", &context).unwrap();

        let mut response = Response::with((status::Ok, template));

        // Setting ContentType
        set_content_type(&mut response);
        Ok(response)
    }

    fn page(req: &mut Request) -> IronResult<Response> {
        let tera = compile_templates!("templates/**/*");
        let context = workbench_common_context();
        let ref page = req.extensions.get::<Router>()
            .unwrap()
            .find("page")
            .unwrap_or("404");
        let template = tera.render(*page, &context).unwrap();

        // Setting ContentType
        let mut response = Response::with((status::Ok, template));
        set_content_type(&mut response);
        Ok(response)
    }

    let mut router = Router::new();

    router.get("/", greeting, "index");
    router.get("/:page", page, "query");

    let mut mount = Mount::new();
    mount
        .mount("/", router)
        .mount("/static/", Static::new(Path::new("static")));

    println!("Running server on http://127.0.0.1:3000/");
    println!("Press Ctrl+C to stop server");
    Iron::new(mount).http("127.0.0.1:3000").unwrap();
}
