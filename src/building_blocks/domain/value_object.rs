pub trait ValueObject<PropsType: Eq> {
    /// Value objects should always be non-mutable. Instead of changing the state of the value
    /// object, it should be replaced by a new instance. Complex value objects should be created
    /// using an assembler, builder or essence pattern.
    fn new(props: PropsType) -> Self;

    /// Access the properties.
    fn get_props(&self) -> PropsType;

    /// Value objects are equal if their values are identical.
    ///
    /// A value object is an object whose value is significant. This means that two value objects
    /// with exactly the same value can be considered the same value object and are therefore
    /// interchangeable.
    fn equals(&self, other: impl ValueObject<PropsType>) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::building_blocks::domain::value_object::ValueObject;

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct SomeValueObjectProps {
        value: i8,
    }

    #[derive(Debug, Clone)]
    struct SomeValueObject {
        props: SomeValueObjectProps,
    }

    impl ValueObject<SomeValueObjectProps> for SomeValueObject {
        fn new(props: SomeValueObjectProps) -> Self {
            SomeValueObject { props }
        }
        fn get_props(&self) -> SomeValueObjectProps {
            self.props.clone()
        }
        fn equals(&self, other: impl ValueObject<SomeValueObjectProps>) -> bool {
            self.props == other.get_props()
        }
    }

    #[test]
    fn it_creates_is_created_by_new() {
        // Arrange
        let props = SomeValueObjectProps { value: 100 };

        // Act, Assert
        let actual = SomeValueObject::new(props);
        assert_eq!(actual.props.value, 100);
    }

    #[test]
    fn it_equals_the_other() {
        // Arrange
        let props = SomeValueObjectProps { value: 100 };
        let props_other = SomeValueObjectProps { value: 100 };
        let actual = SomeValueObject::new(props);
        let actual_other = SomeValueObject::new(props_other);

        // ACT
        let actual_result = actual.equals(actual_other.clone());
        assert_eq!(actual_result, true);
        assert_eq!(actual.get_props().value, actual_other.get_props().value);
    }

    #[test]
    fn it_equals_not_the_other() {
        // Arrange
        let props = SomeValueObjectProps { value: -128 };
        let props_other = SomeValueObjectProps { value: 127 };
        let actual = SomeValueObject::new(props);
        let actual_other = SomeValueObject::new(props_other);

        // ACT
        let actual_result = actual.equals(actual_other.clone());
        assert_eq!(actual_result, false);
        assert_ne!(actual.get_props().value, actual_other.get_props().value);
    }
}
