// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, SyncSender};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, String> {
        let (response_sender, response_reciever) = std::sync::mpsc::sync_channel(1);
        self.sender.try_send(Command::Insert{
            draft: draft,
            response_channel: response_sender
        }).expect("Server is down and/or backedup");

        Ok(response_reciever.recv().expect("Error in inserting ticket"))
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, String> {
        let (response_sender, response_reciever )= std::sync::mpsc::sync_channel(1);
        self.sender.try_send(Command::Get {
            id: id,
            response_channel: response_sender
        }).expect("Server should be able to take requests");

        Ok(response_reciever.recv().expect("Error in returning ticket"))
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let( sender, receiver) = std::sync::mpsc::sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient {
        sender: sender
    }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<crate::store::TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<crate::data::Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                response_channel.try_send(id).expect("Response queue should be empty")
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                response_channel.try_send(ticket.cloned()).expect("Response queue should be empty")
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
