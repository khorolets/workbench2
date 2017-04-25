//! UTILS
// Helpers for Workbench2

use chrono::{UTC};
use iron::Response;

use hyper::header::{ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};

use tera::Context;

use filters;

pub fn set_content_type(response: &mut Response) {
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
    context.add(
        "dummy",
        &""
    );
    context
}

pub fn render_template(filename: &str) -> String {
    //! Renders a template
    let mut tera = compile_templates!("templates/**/*.html");
    tera.register_filter("range", filters::range);
    let context = workbench_common_context();
    let template = tera.render(&filename, &context)
        .unwrap_or("Error in rendering a template".to_string());
    template
}
