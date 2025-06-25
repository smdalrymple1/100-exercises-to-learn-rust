pub const DEFAULT_DESCRIPTION : &str = "No description provided.";
pub const MAX_SIZE : usize= 501;

#[derive(thiserror::Error,Debug,PartialEq)]
pub enum TicketDescriptionError {
    #[error("Description cannot be longer than {MAX_SIZE} bytes")]
    TooLong,
}


#[derive(Debug,PartialEq,Clone)]
pub struct TicketDescription{
    text: String
}

impl TicketDescription {
    pub fn get_text(&self) -> &str {
        self.text.as_str()        
    }
}


impl TryFrom<String> for TicketDescription {
    type Error = TicketDescriptionError;
    fn try_from(description: String) -> Result<Self,Self::Error> {
        match description.len() {
            0 => Ok(TicketDescription{text: DEFAULT_DESCRIPTION.to_string()}),
            1..MAX_SIZE => Ok(TicketDescription{text: description}),
            MAX_SIZE.. =>Err(TicketDescriptionError::TooLong) 
        }
    }
}

impl TryFrom<&str> for TicketDescription {
    type Error = TicketDescriptionError;
    fn try_from(description: &str) -> Result<Self,Self::Error> {
        Self::try_from(description.to_string())
    }
}