#![crate_id = "couch#0.1-pre"]

#![comment = "Relaxing CouchDB client in Rust"]
#![license = "MIT/ASL2"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

extern crate http;
extern crate uuid;
extern crate semver;
extern crate serialize;

use uuid::Uuid;
use semver::Version;
use http::client::RequestWriter;
use http::method::{Get,Put,Delete};
use std::io::BufReader;
use std::io;
use serialize::{json, Encodable, Decodable};
use serialize::json::Json;
use serialize::json::Encoder;
use serialize::json::from_reader;
use std::io::IoResult;

#[deriving(Encodable,Decodable)]
pub struct VendorInfo {
  version: ~str,
  name: ~str
}

#[deriving(Encodable,Decodable)]
pub struct Info {
  couchdb: ~str,
  uuid: Uuid,
  version: ~str,
  vendor: VendorInfo
}

#[deriving(Encodable,Decodable)]
pub struct Okay {
  ok: bool
}

impl Info {
  pub fn version(self) -> Version {
    semver::parse(self.version).unwrap()
  }

  pub fn message(self) -> ~str {
    self.couchdb
  }
}

#[deriving(Clone)]
pub struct Server {
  host: ~str
}

impl Server {
  pub fn new(host: ~str) -> Server {
    Server { host: host }
  }

  fn get(&self, path: ~str) -> IoResult<Vec<u8>> {
    let request: RequestWriter =
              RequestWriter::new(Get,
              from_str(self.host + path).expect("Invalid URL :-("))
              .unwrap();
    let mut response = request.read_response().unwrap();
    response.read_to_end()
  }

  fn put(&self, path: ~str, body: Option<&[u8]>) -> IoResult<Vec<u8>> {
    let mut request: RequestWriter =
              RequestWriter::new(Put,
              from_str(self.host + path).expect("Invalid URL :-("))
              .unwrap();

    match body {
      Some(body) => {
        request.headers.content_length = Some(body.len());
        request.write(body);
      },
      None => {}
    }

    let mut response = request.read_response();
    response.unwrap().read_to_end()
  }

  fn delete(& self, path: ~str) -> IoResult<Vec<u8>> {
    let request: RequestWriter =
              RequestWriter::new(Delete,
              from_str(self.host + path).expect("Invalid URL :-("))
              .unwrap();
    let mut response = request.read_response().unwrap();
    response.read_to_end()
  }

  pub fn create_database(&mut self, name: ~str) -> Database {
    let body = self.put(~"/" + name.clone(), None).unwrap();

    let mut reader = BufReader::new(body.as_slice());
    let json = from_reader(&mut reader);
    let mut decoder = json::Decoder::new(json.unwrap());

    let result = Decodable::decode(&mut decoder);
    let ok: Okay = result.unwrap();
    if ok.ok {
      Database { name: name, server: ~self.clone() }
    } else {
      fail!("(C)ouch")
    }
  }

  pub fn delete_database(&mut self, name: ~str) -> bool {
    let result = self.delete(~"/" + name.clone());

    match result {
      Ok(_) => { true },
      Err(_) => { false }
    }
    //let mut reader = BufReader::new(body.as_slice());
    //let json = from_reader(&mut reader);
    //let mut decoder = json::Decoder::new(json.unwrap());
    //
    //let result = Decodable::decode(&mut decoder);

  }

  pub fn info(self) -> Info {
    let body = self.get(~"/").unwrap();

    let mut reader = BufReader::new(body.as_slice());
    let json = from_reader(&mut reader);
    let mut decoder = json::Decoder::new(json.unwrap());

    let result = Decodable::decode(&mut decoder);
    result.unwrap()
  }
}

pub struct Database {
  name: ~str,
  server: ~Server
}

pub trait Document {
  fn id(&self) -> ~str;
}

impl Database {
  pub fn name(self) -> ~str {
    self.name
  }

  pub fn put<'a, T: Document + Encodable<json::Encoder<'a>,io::IoError>>(&mut self, document: &T) {
    let url = format!("/{:s}/{:s}", self.name, document.id());
    let encoded_doc = json::Encoder::buffer_encode(document);
    self.server.put(url, Some(encoded_doc.as_slice()));
  }
}