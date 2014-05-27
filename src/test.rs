extern crate couch;
extern crate http;
extern crate serialize;

#[cfg(test)]
mod test {
  use couch::{Server,Document};

  #[deriving(Encodable,Decodable)]
  struct TestDocument {
    _id: String,
    body: String
  }

  impl Document for TestDocument {
    fn id(&self) -> String {
      self._id.clone()
    }
  }

  #[test]
  fn speak_to_the_couch() {
    let server = Server::new(String::from_str("http://localhost:5984"));
    let info = server.info();
    assert_eq!(info.message(), "Welcome".to_owned());
  }

  #[test]
  fn create_database() {
    let mut server = Server::new(String::from_str("http://localhost:5984"));
    server.delete_database("created_by_couch".to_owned());
    let database = server.create_database("created_by_couch".to_owned());
    assert_eq!(database.name(), "created_by_couch".to_owned());
  }

  #[test]
  fn create_document() {
    let mut server = Server::new(String::from_str("http://localhost:5984"));
    server.delete_database("create_doc".to_owned());
    let mut database = server.create_database("create_doc".to_owned());
    let test_doc = &TestDocument { _id: "test".to_owned(), body: "test".to_owned() };
    database.put(test_doc);
  }
}