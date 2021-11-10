use crate::building_blocks::domain::entity::Entity;

pub trait DomainAggregate<Root, Identity, DomainEntityProps> where Root: Entity<Identity, DomainEntityProps> {
    fn get_root(&self) -> Root;
}