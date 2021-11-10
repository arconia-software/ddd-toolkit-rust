use ddd_toolkit::building_blocks::domain::value_object::ValueObject;
use crate::building_blocks::domain::value_object::DomainValueObject;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct MyValueObjectProps {
    pub value: i32
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct MyValueObject {
    props: MyValueObjectProps
}

impl ValueObject<MyValueObjectProps> for MyValueObject {
    fn new(props: MyValueObjectProps) -> Self {
        MyValueObject { props }
    }

    fn get_props(&self) -> MyValueObjectProps {
        MyValueObjectProps {
            value: self.props.value
        }
    }

    fn equals(&self, other: impl ValueObject<MyValueObjectProps>) -> bool {
        self.props == other.get_props()
    }
}

fn main() {}