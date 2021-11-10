mod mocks;

#[cfg(test)]
mod tests {
    use ddd_toolkit::building_blocks::domain::entity::Entity;
    use ddd_toolkit::building_blocks::domain::identity::DomainIdentity;
    use aggregate_root_tests::entity::{MyDomainId, MyEntity, MyEntityProps};

    #[test]
    fn it_creates_is_created_by_new() {
        // Arrange
        let identity = MyDomainId::new();
        let props = MyEntityProps { value: 100 };

        // Act, Assert
        let actual = MyEntity::new(identity.clone(), props);
        assert_eq!(actual.get_identity().get_id(), identity.get_id());
        assert_eq!(actual.props.value, 100);
    }

    #[test]
    fn it_is_the_same_as_other() {
        // Arrange
        let id = MyDomainId::new();
        let props = MyEntityProps { value: -128 };
        let id_other = id.clone();
        let props_other = MyEntityProps { value: 127 };
        let actual = MyEntity::new(id, props);
        let actual_other = MyEntity::new(id_other, props_other);

        // Act
        let actual_result = actual.is_same_as(actual_other);

        // Assert
        assert_eq!(actual_result, true);
    }

    #[test]
    fn it_is_not_the_same_as_other() {
        // Arrange
        let id = MyDomainId::new();
        let props = MyEntityProps { value: 127 };
        let id_other = MyDomainId::new();
        let props_other = MyEntityProps { value: 127 };
        let actual = MyEntity::new(id, props);
        let actual_other = MyEntity::new(id_other, props_other);

        // Act
        let actual_result = actual.is_same_as(actual_other);

        // Assert
        assert_eq!(actual_result, false);
    }
}
