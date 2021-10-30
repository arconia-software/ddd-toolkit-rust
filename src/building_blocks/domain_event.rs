pub trait DomainEvent {
    type DateTime;
    type PublisherId;
    type Payload;
    fn new(publisher_id: Self::PublisherId, payload: Self::Payload) -> Self where Self: Sized;
    fn get_date_time_occurred(&self) -> Self::DateTime;
    fn get_publisher_id(&self) -> Self::PublisherId;
    fn get_payload(&self) -> Self::Payload;
    fn get_event_name(&self) -> &str;
}

