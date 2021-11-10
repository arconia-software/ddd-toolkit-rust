mod mocks;
pub mod entity;

#[cfg(test)]
mod tests {
    use ddd_toolkit::examples::aggregate_root::{MyAggregateRootIdentity, MyAggregateRootProps};
    use ddd_toolkit::building_blocks::domain::entity::Entity;
    use ddd_toolkit::building_blocks::domain::identity::DomainIdentity;
    use ddd_toolkit::building_blocks::domain::value_object::ValueObject;

    use crate::building_blocks::domain_aggregate_root::AggregateRoot;
    use crate::building_blocks::domain_entity::DomainEntity;
    use crate::mocks::entity::{MyDomainId, MyEntity, MyEntityProps};
    use crate::mocks::value_object::{MyValueObject, MyValueObjectProps};

    #[derive(Debug, Eq, PartialEq, Clone)]
    fn test() {

        // Arrange
        let identity = MyDomainId::new();
        let value_object_props = MyValueObjectProps { value: 100 };
        let value_object = MyValueObject::new(value_object_props);
        let root_entity = MyEntity::new(identity.clone(), props);

        // Act
        let props = MyAggregateRootProps { value_object };
        let aggregate_root = MyAggregateRootIdentity::new(identity, props);

        // Assert
        assert_eq!(aggregate_root.props.value_object.get_props().value, 100)

    }
}
