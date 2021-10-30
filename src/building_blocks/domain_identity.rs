pub trait DomainIdentity {
    type Id;
    fn new() -> Self;
    fn new_from_existing(value: Self::Id) -> Self;
    fn get_id(&self) -> Self::Id;
}