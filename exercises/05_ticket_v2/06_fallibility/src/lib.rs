#[derive(Debug, PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(title: String, description: String, status: Status) -> Result<Ticket,String> {
        if title.is_empty() {
            Err("Title cannot be empty".into())
        } 
        else if title.len() > 50 {
            Err("Title cannot be longer than 50 bytes".into())
        }
        else if description.is_empty() {
            Err("Description cannot be empty".into())
        }
        else if description.len() > 500 {
            Err("Description cannot be longer than 500 bytes".into())
        }else {
            Ok(
                Ticket {
                    title,
                    description,
                    status,
                }
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    fn title_cannot_be_empty() {
        let error = Ticket::new("".into(), valid_description(), Status::ToDo).unwrap_err();
        assert_eq!(error, "Title cannot be empty");
    }

    #[test]
    fn description_cannot_be_empty() {
        let error = Ticket::new(valid_title(), "".into(), Status::ToDo).unwrap_err();
        assert_eq!(error, "Description cannot be empty");
    }

    #[test]
    fn title_cannot_be_longer_than_fifty_chars() {
        let error =
            Ticket::new(overly_long_title(), valid_description(), Status::ToDo).unwrap_err();
        assert_eq!(error, "Title cannot be longer than 50 bytes");
    }

    #[test]
    fn description_cannot_be_longer_than_500_chars() {
        let error =
            Ticket::new(valid_title(), overly_long_description(), Status::ToDo).unwrap_err();
        assert_eq!(error, "Description cannot be longer than 500 bytes");
    }
}
