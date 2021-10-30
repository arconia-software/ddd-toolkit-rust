use chrono::{DateTime, Utc};
use ddd_toolkit::building_blocks::domain_event::{DomainEvent};
use crate::util::entity::MyDomainId;

pub enum MyEvent {
    MyEvent(MyDomainEvent),
    MyOtherEvent(MyOtherDomainEvent)
}

#[derive(Debug, Clone)]
pub struct MyDomainEventPayload {
    pub message: String
}

#[derive(Debug, Clone)]
pub struct MyDomainEvent {
    date_time_occurred: DateTime<Utc>,
    entity_id: MyDomainId,
    payload: MyDomainEventPayload
}

impl DomainEvent for MyDomainEvent {
    type DateTime = DateTime<Utc>;
    type PublisherId = MyDomainId;
    type Payload = MyDomainEventPayload;

    fn new(publisher_id: Self::PublisherId, payload: Self::Payload) -> Self {
        MyDomainEvent {
            date_time_occurred: Utc::now(),
            entity_id: publisher_id,
            payload
        }
    }

    fn get_date_time_occurred(&self) -> Self::DateTime {
        self.date_time_occurred
    }

    fn get_publisher_id(&self) -> Self::PublisherId {
        self.entity_id.clone()
    }

    fn get_payload(&self) -> Self::Payload {
        self.payload.clone()
    }

    fn get_event_name(&self) -> &str {
        std::any::type_name::<Self>()
    }

}

#[derive(Debug, Clone)]
pub struct MyOtherDomainEventPayload {
    pub token: String
}

#[derive(Debug, Clone)]
pub struct MyOtherDomainEvent {
    date_time_occurred: DateTime<Utc>,
    entity_id: MyDomainId,
    payload: MyOtherDomainEventPayload
}

impl DomainEvent for MyOtherDomainEvent {
    type DateTime = DateTime<Utc>;
    type PublisherId = MyDomainId;
    type Payload = MyOtherDomainEventPayload;

    fn new(publisher_id: Self::PublisherId, payload: Self::Payload) -> Self {
        MyOtherDomainEvent {
            date_time_occurred: Utc::now(),
            entity_id: publisher_id,
            payload
        }
    }

    fn get_date_time_occurred(&self) -> Self::DateTime {
        self.date_time_occurred
    }

    fn get_publisher_id(&self) -> Self::PublisherId {
        self.entity_id.clone()
    }

    fn get_payload(&self) -> Self::Payload {
        self.payload.clone()
    }

    fn get_event_name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}