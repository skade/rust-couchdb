#![crate_id = "couch#0.1-pre"]

#![comment = "Relaxing CouchDB client in Rust"]
#![license = "MIT/ASL2"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

extern crate http;
extern crate uuid;
extern crate semver;
extern crate serialize;
extern crate debug;

use uuid::Uuid;
use semver::Version;
use http::client::RequestWriter;
use http::method::{Get,Put,Delete};
use std::io::BufReader;
use std::io;
use serialize::{json, Encodable, Decodable};
use serialize::json::Encoder;
use serialize::json::from_reader;
use std::io::IoResult;

#[deriving(Encodable,Decodable)]
pub struct VendorInfo {
  version: String,
  name: String
}

#[deriving(Encodable,Decodable)]
pub struct Info {
  couchdb: String,
  uuid: Uuid,
  version: String,
  vendor: VendorInfo
}

#[deriving(Encodable,Decodable)]
pub struct Okay {
  ok: bool
}

impl Info {
  pub fn version(self) -> Version {
    semver::parse(self.version.as_slice()).unwrap()
  }

  pub fn message(self) -> String {
    self.couchdb
  }
}

#[deriving(Clone)]
pub struct Server {
  host: String
}

impl Server {
  pub fn new(host: String) -> Server {
    Server { host: host }
  }

  fn get(&self, path: String) -> IoResult<Vec<u8>> {
    let request: RequestWriter =
              RequestWriter::new(Get,
              from_str(self.host.clone().append(path.as_slice()).as_slice()).expect("Invalid URL :-("))
              .unwrap();
    let response = request.read_response();
    match response {
      Ok(mut res) => { res.read_to_end() },
      Err(error) => { fail!("(C)ouch! {:?}", error) }
    }
  }

  fn put(&self, path: String, body: Option<&[u8]>) -> IoResult<Vec<u8>> {
    let mut request: RequestWriter =
              RequestWriter::new(Put,
              from_str(self.host.clone().append(path.as_slice()).as_slice()).expect("Invalid URL :-("))
              .unwrap();

    match body {
      Some(body) => {
        request.headers.content_length = Some(body.len());
        request.write(body);
      },
      None => {}
    }

    let response = request.read_response();
    match response {
      Ok(mut res) => { res.read_to_end() },
      Err(error) => { fail!("(C)ouch! {:?}", error) }
    }
  }

  fn delete(&self, path: String) -> IoResult<Vec<u8>> {
    let request: RequestWriter =
              RequestWriter::new(Delete,
              from_str(self.host.clone().append(path.as_slice()).as_slice()).expect("Invalid URL :-("))
              .unwrap();
    let response = request.read_response();
    match response {
      Ok(mut res) => { res.read_to_end() },
      Err(error) => { fail!("(C)ouch! {:?}", error) }
    }
  }

  pub fn create_database(&mut self, name: String) -> Database {
    let body = self.put(String::from_str("/").append(name.as_slice()), None).unwrap();

    let mut reader = BufReader::new(body.as_slice());
    let json = from_reader(&mut reader);
    let mut decoder = json::Decoder::new(json.unwrap());

    let result = Decodable::decode(&mut decoder);
    let ok: Okay = result.unwrap();
    if ok.ok {
      Database { name: name, server: self.clone() }
    } else {
      fail!("(C)ouch")
    }
  }

  pub fn delete_database(&mut self, name: String) -> bool {
    let result = self.delete(String::from_str("/").append(name.as_slice()));

    // TODO: Yes, don't do that, but for the sake of a quick hack
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
    let body = self.get(String::from_str("/")).unwrap();

    let mut reader = BufReader::new(body.as_slice());
    let json = from_reader(&mut reader);
    let mut decoder = json::Decoder::new(json.unwrap());

    let result = Decodable::decode(&mut decoder);
    result.unwrap()
  }
}

pub struct Database {
  name: String,
  server: Server
}

pub trait Document {
  fn id(&self) -> String;
}

impl Database {
  pub fn name(self) -> String {
    self.name
  }

  pub fn put<'a, T: Document + Encodable<json::Encoder<'a>,io::IoError>>(&mut self, document: &T) {
    let url = format!("/{:s}/{:s}", self.name, document.id());
    let encoded_doc = json::Encoder::buffer_encode(document);
    self.server.put(url, Some(encoded_doc.as_slice()));
  }
}
