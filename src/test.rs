extern crate couch;
extern crate http;
extern crate serialize;

#[cfg(test)]
mod test {
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

  #[test]
  fn speak_to_the_couch() {
    let server = Server::new(~"http://localhost:5984");
    let info = server.info();
    assert_eq!(info.message(), ~"Welcome");
  }

  #[test]
  fn create_database() {
    let mut server = Server::new(~"http://localhost:5984");
    server.delete_database(~"created_by_couch");
    let database = server.create_database(~"created_by_couch");
    assert_eq!(database.name(), ~"created_by_couch");
  }

  #[test]
  fn create_document() {
    let mut server = Server::new(~"http://localhost:5984");
    server.delete_database(~"create_doc");
    let ~mut database = ~server.create_database(~"create_doc");
    let test_doc = &TestDocument { _id: ~"test", body: ~"test" };
    database.put(test_doc);
  }
}