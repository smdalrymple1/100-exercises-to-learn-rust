use std::sync::mpsc::{Receiver, Sender};
use crate::store::TicketStore;

pub mod data;
pub mod store;

// Refer to the tests to understand the expected schema.
pub enum Command {
    Insert { 
        draft: crate::data::TicketDraft,
        response_sender: Sender<crate::store::TicketId>
     },
    Get { 
        id: crate::store::TicketId,
        response_sender: Sender<Option<crate::data::Ticket>>
     }
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: handle incoming commands as expected.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert { draft: ticket_draft, response_sender: response_sender }) => {
                response_sender.send(store.add_ticket(ticket_draft));
            }
            Ok(Command::Get {
                id: ticket_id, response_sender: response_sender
            }) => {
                response_sender.send(store.get(ticket_id).cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break
            },
        }
    }
}
