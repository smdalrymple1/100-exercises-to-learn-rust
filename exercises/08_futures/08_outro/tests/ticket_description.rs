use outro_08::ticket_description::{TicketDescription, TicketDescriptionError,DEFAULT_DESCRIPTION,MAX_SIZE};

#[test]
fn try_from_works(){
    let description_text = "This is a really important ticket. Put your best people on it";
    let description_field = TicketDescription::try_from(description_text);
    match description_field {
        Ok(created_description) => assert_eq!(created_description.get_text(), description_text),
        _ => panic!("Unexpected Error returned")
    }
}
    
#[test]
fn empty_description_gets_default(){
    let description_text = "";
    let description_field = TicketDescription::try_from(description_text);
    match description_field {
        Ok(created_description) => assert_eq!(
            created_description.get_text(),
            DEFAULT_DESCRIPTION
        ),
        _ => panic!("Unexpected Error returned")
    };
}

#[test]
fn long_description_raises_error(){
    let description_text = ['A';MAX_SIZE].iter().collect::<String>();
    let description_field = TicketDescription::try_from(description_text);
    match description_field {
        Err(TicketDescriptionError::TooLong) => (),
        _ => panic!("Unexpected Type Returned")
    }
}
