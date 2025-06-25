pub const MAX_SIZE :usize = 51;

#[derive(thiserror::Error,Debug,PartialEq)]
pub enum TicketTitleError {
    #[error("Title cannot be longer than {MAX_SIZE} bytes")]
    TooLong,
    #[error("Title cannot be empty")]
    Empty
}

#[derive(Debug,PartialEq,Clone)]
pub struct TicketTitle{
    text: String
}

impl TicketTitle {
    pub fn get_text(&self) -> &str {
        self.text.as_str()
    }
}

impl TryFrom<String> for TicketTitle {
    type Error = TicketTitleError;
    fn try_from(title: String) -> Result<Self,Self::Error>{
        match title.len() {
            0 => Err(TicketTitleError::Empty),
            0..MAX_SIZE => Ok(TicketTitle {
                text: title
            }),
            MAX_SIZE.. => Err(TicketTitleError::TooLong),
        }
    }
}

impl TryFrom<&str> for TicketTitle {
    type Error = TicketTitleError;
    fn try_from(title: &str) -> Result<Self,Self::Error>{
        Self::try_from(title.to_string())
    }
}