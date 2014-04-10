# rust-couchdb

A relaxing Rust lib.

## Prerequisites

Rust, as fresh as possible, no release versions. Tested against 7fbcb4.

## Building

```
git clone --recursive https://github.com/skade/rust-couchdb.git
cd rust-couchdb
make
make test
```

## Synopsis

```rust
use couch::{Server,Document};

#[deriving(Encodable,Decodable)]
struct TestDocument {
  _id: ~str,
  body: ~str
}

impl Document for TestDocument {
  fn id(&self) -> ~str {
    self._id.clone()
  }
}

fn main() {
  let mut server = Server::new(~"http://localhost:5984");
  let ~mut database = ~server.create_database(~"create_doc");
  let test_doc = &TestDocument { _id: ~"test", body: ~"test" };
  database.put(test_doc);
}
```

## Conceptual

Make sure you use mutable pointers to Server and Database if you want to change anything. The API makes sure that immutable handles are read-only.

## Done

* Reading server info
* Creating databases
* Deleting databases
* Creating documents using the PUT api

## TODO

Everything else.
