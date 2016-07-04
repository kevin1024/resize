extern crate iron;
extern crate hyper;
extern crate image;

use iron::prelude::*;
use iron::status;
use std::io::Read;
use hyper::client::Client;

fn resize(buffer: &Vec<u8>) -> Vec<u8> {
    let mut output = Vec::new();
    match image::load_from_memory(buffer) {
      Ok(img) => {
          let resized = img.resize(100,100, image::FilterType::Triangle);
          let color = img.color();
          let bytes = resized.raw_pixels();
          image::jpeg::JPEGEncoder::new(&mut output).encode(&bytes, 100, 100, color);
          return output;
      },
      Err(_) => panic!("Error reading image")
    
    }
}

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
        let resized = resize(&data);
        Ok(Response::with((status::Ok, resized)))
    }).http("localhost:3000").unwrap();
}
