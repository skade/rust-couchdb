# rust-couchdb

A relaxing Rust lib.

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