// TODO: `easy_ticket` should panic when the title is invalid.
//   When the description is invalid, instead, it should use a default description:
//   "Description not provided".
fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    match Ticket::new(title.clone(), description.clone(), status.clone()) {
        Ok(ticket) => ticket,
        Err((error_type,message)) => {
            match error_type {
                BadTicketError::BadTitle => panic!("{}",message),
                BadTicketError::BadDescription => Ticket::new(title.clone(),"Description not provided".into(),status.clone()).unwrap()
            } 
        }
    } 
}

#[derive(Debug,Clone,Copy)]
enum BadTicketError {
    BadTitle,
    BadDescription
}

#[derive(Debug, PartialEq, Clone)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(title: String, description: String, status: Status) -> Result<Ticket, (BadTicketError,String)> {
        if title.is_empty() {
            return Err((BadTicketError::BadTitle,"Title cannot be empty".into()));
        }
        if title.len() > 50 {
            return Err((BadTicketError::BadTitle, "Title cannot be longer than 50 bytes".into()));
        }
        if description.is_empty() {
            return Err((BadTicketError::BadDescription,"Description cannot be empty".into()));
        }
        if description.len() > 500 {
            return Err((BadTicketError::BadDescription, "Description cannot be longer than 500 bytes".into()));
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        easy_ticket("".into(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_empty() {
        let ticket = easy_ticket(valid_title(), "".into(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        easy_ticket(overly_long_title(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_too_long() {
        let ticket = easy_ticket(valid_title(), overly_long_description(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }
}
