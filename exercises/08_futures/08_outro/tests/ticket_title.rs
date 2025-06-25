use outro_08::ticket_title::{MAX_SIZE,TicketTitle,TicketTitleError};

#[test]
fn try_from_works() {
    let title = "Task123";
    let validated_title = TicketTitle::try_from(title);
    match validated_title {
        Ok(title_object) => assert_eq!(title_object.get_text(),title),
        _ => panic!("Unexpected Errr")
    }
}

#[test]
fn try_from_returns_error_for_empty_string(){
    let title = "";
    let validated_title = TicketTitle::try_from(title);
    match validated_title {
        Err(TicketTitleError::Empty) => (),
        _ => panic!("Unexpected result for empty title")
    }
}

#[test]
fn try_from_returns_error_for_too_long_string(){
    let title = ['A';MAX_SIZE].iter().collect::<String>();
    let validated_title = TicketTitle::try_from(title);
    match validated_title {
        Err(TicketTitleError::TooLong) => (),
        _ => panic!("Unexpected result for a title that's too long")
    }
}