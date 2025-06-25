#[derive(Hash,Eq,PartialEq,Clone,Copy,Debug)]
pub struct TicketId {
    raw_id: u64
}

impl TicketId {
    pub fn get_raw_id(&self) -> u64 {
        self.raw_id
    }
}

impl From<u64> for TicketId {
    fn from(raw_id: u64) -> Self {
        TicketId{
            raw_id: raw_id
        }
    }
}

impl From<&u64> for TicketId {
    fn from(raw_id: &u64) -> Self{
        Self::from(*raw_id)
    }
}