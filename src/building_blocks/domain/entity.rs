pub trait Entity<Identity, DomainEntityProps>  {

    fn new(id: Identity, props: DomainEntityProps) -> Self;

    fn get_identity(&self) -> Identity;

    /// Entities are equal if their IDs equal each other.
    fn is_same_as(&self, other: impl Entity<Identity, DomainEntityProps>) -> bool;
}

pub trait EventDispatchable {
    type Events;
    fn add_event_for_dispatch(&mut self, event: Self::Events);
}

