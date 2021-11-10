use crate::building_blocks::domain::entity::Entity;

/// The root entity of an aggregate
pub trait DomainAggregateRoot<Identity, DomainAggregateRootProps>: Entity<Identity, DomainAggregateRootProps> {
    fn get_root_id(&self) -> Identity;
}