// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.

#[derive(PartialEq, Debug, Clone)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug,thiserror::Error)]
pub enum StatusError {
    #[error("Invalid Status Provided")]
    InvalidStatus
}

impl TryFrom<String> for Status {
    type Error = StatusError;

    fn try_from(value: String) -> Result<Status,StatusError> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for Status {
    type Error = StatusError;

    fn try_from(value: &str) -> Result<Status,StatusError> {
        match value.to_lowercase().as_str() {
            "todo" => Ok(Status::ToDo),
            "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            _ => Err(StatusError::InvalidStatus)
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("ToDO").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_invalid() {
        let status = Status::try_from("Invalid");
        assert!(status.is_err());
    }
}
