extern crate iron;
extern crate hyper;

use iron::prelude::*;
use iron::status;
use std::io::Read;
use hyper::client::Client;

fn download(source_url: &str) -> Vec<u8> {
    let client = Client::new();
    let mut response_body = Vec::new();
    client.get(&source_url.to_owned())
                 .send()
                 .unwrap()
                 .read_to_end(&mut response_body)
                 .unwrap();
    return response_body;
}

fn main() {
    Iron::new(|_: &mut Request| {
        let data = download("http://apod.nasa.gov/apod/image/1607/catseye3_hst_960.jpg");
        Ok(Response::with((status::Ok, data)))
    }).http("localhost:3000").unwrap();
}
