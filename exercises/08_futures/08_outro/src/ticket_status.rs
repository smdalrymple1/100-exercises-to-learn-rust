#[derive(thiserror::Error, Debug, PartialEq)]
pub enum TicketStatusError {
    #[error("Status does not exist")]
    NonExistant
}

#[derive(Debug,PartialEq,Clone)]
pub enum TicketStatus {
    Todo,
    InProgress,
    Done
}

impl TryFrom<String> for TicketStatus {
    type Error = TicketStatusError;
    fn try_from(status: String) -> Result<Self,Self::Error> {
        Self::try_from(status.as_str())
    }
}

impl TryFrom<&str> for TicketStatus {
    type Error = TicketStatusError;
    fn try_from(status: &str) -> Result<Self,Self::Error> {
        match status.to_lowercase().as_str() {
            "todo" => Ok(TicketStatus::Todo),
            "inprogress" => Ok(TicketStatus::InProgress),
            "done" => Ok(TicketStatus::Done),
            _ => Err(TicketStatusError::NonExistant)
        }
    }
}