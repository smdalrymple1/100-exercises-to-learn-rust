use std::collections::HashMap;

use crate::ticket::{Ticket,TicketDraft,TicketUpdate};
use crate::ticket_id::{TicketId};
use crate::ticket_status::{TicketStatus};

#[derive(Debug,PartialEq)]
pub enum TicketStoreError{
    TicketNotFound
}

pub struct TicketStore {
    count: u64,
    tickets: HashMap<TicketId,Ticket>
}

impl TicketStore {
    pub fn new() -> Self{
        TicketStore {
            count: 0,
            tickets: HashMap::new()
        }
    }

    pub fn add(&mut self, draft : TicketDraft) -> TicketId{
        let ticket_id = TicketId::from(self.count);
        self.count += 1;
        self.tickets.insert(
            ticket_id,
            Ticket {
                id: ticket_id,
                title: draft.title,
                description: draft.description,
                status: TicketStatus::Todo
            }
        );
        ticket_id
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.tickets.get(&id)
    }

    pub fn update(&mut self, update: TicketUpdate) -> Result<&Ticket,TicketStoreError> {
        match self.tickets.get_mut(&update.id) {
            Some(ticket) => {
                if let Some(description) = update.description { ticket.description = description; }
                if let Some(status) = update.status { ticket.status = status; }
                if let Some(title) = update.title {ticket.title = title; }
                Ok(ticket)
            }
            None => { Err(TicketStoreError::TicketNotFound)}
        }
    }
}