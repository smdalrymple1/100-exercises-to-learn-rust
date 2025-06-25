use outro_08::ticket_status::{TicketStatus,TicketStatusError};

#[test]
fn strings_convert_to_status(){
    let todo = "todo";
    let inprogress = "inprogress";
    let done = "done";
    let todo_status = TicketStatus::try_from(todo).unwrap();
    let inprogress_status = TicketStatus::try_from(inprogress).unwrap();
    let done_status = TicketStatus::try_from(done).unwrap();
    assert_eq!(TicketStatus::Todo,todo_status);
    assert_eq!(TicketStatus::InProgress,inprogress_status);
    assert_eq!(TicketStatus::Done,done_status);
}

#[test]
fn uppercase_strings_are_converted_to_status(){
    let todo = "TODO";
    let inprogress = "INPROGRESS";
    let done = "DONE";
    let todo_status = TicketStatus::try_from(todo).unwrap();
    let inprogress_status = TicketStatus::try_from(inprogress).unwrap();
    let done_status = TicketStatus::try_from(done).unwrap();
    assert_eq!(TicketStatus::Todo,todo_status);
    assert_eq!(TicketStatus::InProgress,inprogress_status);
    assert_eq!(TicketStatus::Done,done_status);
}

#[test]
fn invalid_string_returns_error(){
    let random_text = "RandomStatus";
    let invalid_status = TicketStatus::try_from(random_text).unwrap_err();
    assert_eq!(TicketStatusError::NonExistant, invalid_status);
}

#[test]
fn empty_string_returns_error(){
    let empty_text = "";
    let invalid_status = TicketStatus::try_from(empty_text).unwrap_err();
    assert_eq!(TicketStatusError::NonExistant, invalid_status);
}