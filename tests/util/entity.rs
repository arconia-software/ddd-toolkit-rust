use uuid::Uuid;
use ddd_toolkit::building_blocks::domain_entity::DomainEntity;
use ddd_toolkit::building_blocks::domain_identity::DomainIdentity;
use crate::util::event::{MyDomainEvent, MyEvent, MyOtherDomainEvent};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MyDomainId {
    id: Uuid
}

impl DomainIdentity for MyDomainId {
    type Id = Uuid;

    fn new() -> Self {
        let uuid = Uuid::new_v4();
        MyDomainId { id: uuid }
    }

    fn new_from_existing(value: Self::Id) -> Self {
        MyDomainId { id: value }
    }

    fn get_id(&self) -> Self::Id {
        self.id.clone()
    }
}


#[derive(Debug, Copy, Clone)]
pub struct MyEntityProps {
    pub value: i8
}


pub struct MyEntity {
    identity: MyDomainId,
    pub props: MyEntityProps,
    pub(crate) my_domain_events: (Vec<MyDomainEvent>, Vec<MyOtherDomainEvent>)
}

impl DomainEntity for MyEntity {
    type Id = MyDomainId;
    type Props = MyEntityProps;
    type Events = MyEvent;

    fn new(id: MyDomainId, props: MyEntityProps) -> Self {
        MyEntity { identity: id, props, my_domain_events: (vec![], vec![]) }
    }

    fn get_identity(&self) -> MyDomainId {
        self.identity.clone()
    }

    fn is_same_as<T>(&self, other: T) -> bool where T: DomainEntity<Id=Self::Id> {
        self.identity.id == other.get_identity().id
    }

    fn add_event_for_dispatch(&mut self, event: Self::Events) {
        match event {
            MyEvent::MyEvent(event) => {
                self.my_domain_events.0.push(event)
            }
            MyEvent::MyOtherEvent(event) => {
                self.my_domain_events.1.push(event)
            }
        }
    }
}
