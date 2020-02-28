use std::error::Error;
use std::marker::{Send, Sync};
use uuid::Uuid;

#[derive(Debug)]
pub struct ToDo {
  id: Uuid,
  text: String,
}

impl ToDo {
  pub fn new(id: Uuid, text: String) -> ToDo {
    ToDo { id: id, text: text }
  }

  pub fn new_random_id(text: String) -> ToDo {
    ToDo {
      id: Uuid::new_v4(),
      text,
    }
  }

  pub fn id(&self) -> String {
    self.id.to_simple().to_string()
  }

  pub fn text(&self) -> &String {
    &self.text
  }
}

#[juniper::object]
impl ToDo {
  fn id(&self) -> String {
    self.id()
  }

  fn text(&self) -> &String {
    self.text()
  }
}

impl PartialEq for ToDo {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

pub trait Repository<E>: Sync + Send
where
  E: Error,
{
  fn list(&self) -> Result<Vec<ToDo>, E>;
  fn create(&self, todo: ToDo) -> Result<ToDo, E>;
}

#[cfg(test)]
mod test {
  use super::*;
  use uuid::Uuid;

  #[test]
  fn test_id() {
    let todo = ToDo {
      id: Uuid::parse_str("97103fab-1e50-36b7-0c03-0938362b0809").unwrap(),
      text: String::from("sample"),
    };

    assert_eq!(todo.id(), "97103fab1e5036b70c030938362b0809");
    assert_ne!(todo.id(), "97103fab-1e50-36b7-0c03-0938362b0809");
  }

  #[test]
  fn test_text() {
    let todo = ToDo {
      id: Uuid::parse_str("97103fab-1e50-36b7-0c03-0938362b0809").unwrap(),
      text: String::from("sample"),
    };

    assert_eq!(todo.text(), &String::from("sample"));
    assert_ne!(todo.text(), &String::from("not equal"));
  }

  #[test]
  fn test_partial_eq() {
    let id = Uuid::parse_str("97103fab-1e50-36b7-0c03-0938362b0809").unwrap();

    assert_eq!(
      ToDo {
        id: id,
        text: String::from("one")
      },
      ToDo {
        id: id,
        text: String::from("other")
      }
    );

    assert_ne!(
      ToDo {
        id: id,
        text: String::from("one")
      },
      ToDo {
        id: Uuid::parse_str("aaaaaaaa-1e50-36b7-0c03-0938362b0809").unwrap(),
        text: String::from("one")
      }
    );
  }
}
