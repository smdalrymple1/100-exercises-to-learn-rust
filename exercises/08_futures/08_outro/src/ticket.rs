use crate::ticket_description::TicketDescription;
use crate::ticket_id::TicketId;
use crate::ticket_status::TicketStatus;
use crate::ticket_title::TicketTitle;

#[derive(Clone)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription
}

#[derive(PartialEq,Debug)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: TicketStatus
}

#[derive(Clone)]
pub struct TicketUpdate {
    pub id: TicketId,
    pub title: Option<TicketTitle>,
    pub description: Option<TicketDescription>,
    pub status: Option<TicketStatus>
}

