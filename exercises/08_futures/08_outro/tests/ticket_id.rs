use outro_08::ticket_id::TicketId;

#[test]
fn sanity_check(){
    let id :u64 = 101;
    assert_eq!(TicketId::from(id).get_raw_id(),id);
}