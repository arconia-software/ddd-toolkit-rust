pub trait DomainEntity  {
    type Id;
    type Props;
    type Events;

    fn new(id: Self::Id, props: Self::Props) -> Self;

    fn get_identity(&self) -> Self::Id;

    /// Entities are equal if their IDs equal each other.
    fn is_same_as<T>(&self, other: T) -> bool where T: DomainEntity<Id = Self::Id>;

    fn add_event_for_dispatch(&mut self, event: Self::Events);
}

