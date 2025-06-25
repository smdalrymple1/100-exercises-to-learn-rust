use outro_08::ticket::{TicketDraft,TicketUpdate};
use outro_08::ticket_description::{TicketDescription};
use outro_08::ticket_id::{TicketId};
use outro_08::ticket_status::{TicketStatus};
use outro_08::ticket_store::{TicketStore,TicketStoreError};
use outro_08::ticket_title::{TicketTitle};

#[test]
fn can_add_get_modify_and_get_again() {
    let mut ticket_store = TicketStore::new();
    let test_draft = TicketDraft{
        title: TicketTitle::try_from("New Ticket").unwrap(),
        description: TicketDescription::try_from("so many bugs, so little time").unwrap()
    };
    let ticket_id = ticket_store.add(test_draft.clone());
    let stored_ticket = ticket_store.get(ticket_id);
    match stored_ticket {
        Some(actual_ticket) => {
            assert_eq!(actual_ticket.id, ticket_id);
            assert_eq!(actual_ticket.description, test_draft.description);
            assert_eq!(actual_ticket.status, TicketStatus::Todo);
            assert_eq!(actual_ticket.title, test_draft.title);
        },
        None => { panic!("Ticket not found"); }
    }
    let ticket_update = TicketUpdate{
        id: ticket_id,
        description: Some(TicketDescription::try_from("New Description").unwrap()),
        status: Some(TicketStatus::Done),
        title: Some(TicketTitle::try_from("New Title").unwrap())
    };
    let updated_ticket = ticket_store.update(ticket_update.clone());
    match updated_ticket { 
        Ok(actual_ticket) => {
            assert_eq!(ticket_update.id, actual_ticket.id);
            assert_eq!(ticket_update.description.unwrap(), actual_ticket.description);
            assert_eq!(ticket_update.status.unwrap(), actual_ticket.status);
            assert_eq!(ticket_update.title.unwrap(), actual_ticket.title);
        },
        _ => {panic!("Unexpected failure to update")}
    }
}

#[test]
fn get_non_existant_id_returns_none(){
    let ticket_store = TicketStore::new();
    let no_ticket = ticket_store.get(TicketId::from(42));
    assert_eq!(no_ticket, None);
}

#[test]
fn update_non_existant_id_returns_error(){
    let mut ticket_store = TicketStore::new();
    let bad_update = TicketUpdate {
        id: TicketId::from(42),
        description: None,
        status: None,
        title: None
    };
    let bad_update_result = ticket_store.update(bad_update);
    assert_eq!(TicketStoreError::TicketNotFound,bad_update_result.unwrap_err())
}