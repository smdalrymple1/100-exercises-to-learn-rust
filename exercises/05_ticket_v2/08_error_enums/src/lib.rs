#[derive(Debug)]
enum TicketNewError {
    TitleError { message: String },
    DescriptionError { message: String}
}


fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    match Ticket::new(title.clone(), description.clone(), status.clone()) {
        Ok(ticket) => ticket,
        Err(error) => {
            match error {
                TicketNewError::TitleError { message } => panic!("{message}"),
                TicketNewError::DescriptionError { message } => {
                    Ticket::new(title, "Description not provided".into(), status).unwrap()
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
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
    pub fn new(
        title: String,
        description: String,
        status: Status,
    ) -> Result<Ticket, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleError {
                message: "Title cannot be empty".to_string()
            });
        }
        if title.len() > 50 {
            return Err(
                TicketNewError::TitleError{
                    message: "Title cannot be longer than 50 bytes".to_string()
                }
            );
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionError{
                message: "Description cannot be empty".into()
            });
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionError{
                message: "Description cannot be longer than 500 bytes".into()
            });
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
