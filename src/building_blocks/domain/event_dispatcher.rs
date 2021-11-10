pub trait DomainEventDispatcher<AggregateType, EntityType> {
    fn dispatch_events_from_aggregate(aggregate_root: AggregateType);
    fn dispatch_events_from_entity(entity: EntityType);
}