mod mocks;

#[cfg(test)]
mod tests {
    use ddd_toolkit::building_blocks::domain::entity::Entity;
    use ddd_toolkit::building_blocks::domain::event::Event;
    use ddd_toolkit::building_blocks::domain::identity::DomainIdentity;
    use aggregate_root_tests::entity::{MyDomainId, MyEntity, MyEntityProps};
    use ddd_toolkit::examples::event::{MyEvent, MyDomainEvent, MyDomainEventPayload, MyOtherDomainEvent, MyOtherDomainEventPayload};

    fn create_entity() -> MyEntity {
        let id = MyDomainId::new();
        let props = MyEntityProps { value: 55 };
        MyEntity::new(id, props)
    }

    #[test]
    fn it_adds_an_event_to_an_entity() {
        let mut entity = create_entity();
        let event_payload = MyDomainEventPayload {
            message: "Hello world".to_string()
        };
        let event = MyDomainEvent::new(
            entity.get_identity(),
            event_payload
        );
        let other_event_payload = MyOtherDomainEventPayload {
            token: "GrantAccessFor1233".to_string()
        };
        let other_event = MyOtherDomainEvent::new(
            entity.get_identity(),
            other_event_payload
        );

        entity.add_event_for_dispatch(MyEvent::MyEvent(event));
        entity.add_event_for_dispatch(MyEvent::MyOtherEvent(other_event));
    }
}