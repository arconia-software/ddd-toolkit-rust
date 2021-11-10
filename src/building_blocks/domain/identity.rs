pub trait DomainIdentity<IdType> {

    /// Generates a new instance with a random identity value.
    fn new() -> Self;

    /// Create an identity instance from an exising identity value.
    fn new_from_existing(value: IdType) -> Self;

    /// Returns the value of the concrete ID.
    fn get_id(&self) -> IdType;
}