//! FILTERS
//! additional filters for Workbench2
use std::collections::HashMap;
use tera::Result;
use serde_json::value::{Value, to_value};

pub fn range(_: Value, mut args: HashMap<String, Value>) -> Result<Value> {
    //! Filter to provide empty arrays of ``n`` length to duplicate some
    //! parts of markup.
    //! Usage: {{ dummy | range(n=10) }}
    //! There can be any variable instead of ``dummy``

    let number: usize = match args.remove("n") {
        Some(n) => try_get_value!("range", "n", usize, n),
        None => 0
    };

    // let range_array = vec![0; number];
    Ok(to_value(&vec![0; number]).unwrap())
}
