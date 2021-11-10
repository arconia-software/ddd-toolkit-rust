// use std::any::Any;
// use chrono::{DateTime, Utc};
// use uuid::Uuid;
// use ddd_toolkit::building_blocks::domain::aggregate_root::DomainAggregateRoot;
// use ddd_toolkit::building_blocks::domain::entity::Entity;
// use ddd_toolkit::building_blocks::domain::event::Event;
// use ddd_toolkit::building_blocks::domain::identity::DomainIdentity;
// use ddd_toolkit::building_blocks::domain::value_object::ValueObject;
//
// pub struct Order {
//     root_entity: Customer,
//     items: Vec<Item>
// }
//
// impl Order {
//     fn new(root: Customer, items: Vec<Item>) -> Self {
//         return Order { root_entity: root, items}
//     }
// }
//
// /* Identity */
//
// #[derive(Debug, Clone, Eq, PartialEq)]
// pub struct CustomerID {
//     id: Uuid
// }
//
// impl DomainIdentity<Uuid> for CustomerID {
//
//     fn new() -> Self {
//         let uuid = Uuid::new_v4();
//         CustomerID { id: uuid }
//     }
//
//     fn new_from_existing(value: Uuid) -> Self {
//         CustomerID { id: value }
//     }
//
//     fn get_id(&self) -> Uuid {
//         self.id.clone()
//     }
// }
//
// /* Root Entity */
//
// #[derive(Debug, Clone)]
// pub struct CustomerProps {
//     pub value_object: Address
// }
//
// pub struct Customer {
//     identity: CustomerID,
//     pub props: CustomerProps,
//     pub(crate) my_domain_events: Vec<CustomerEvents>
// }
//
// impl DomainAggregateRoot<CustomerID, CustomerProps> for Customer {
//     fn get_root_id(&self) -> CustomerID {
//         self.identity.clone()
//     }
// }
//
// impl Entity<CustomerID, CustomerProps> for Customer {
//     type Events = CustomerEvents;
//
//     fn new(id: CustomerID, props: CustomerProps) -> Self {
//         Customer { identity: id, props, my_domain_events: vec![] }
//     }
//
//     fn get_identity(&self) -> CustomerID {
//         self.identity.clone()
//     }
//
//     fn is_same_as(&self, other: impl Entity<CustomerID, CustomerProps>) -> bool {
//         self.identity.id == other.get_identity().id
//     }
//
//     fn add_event_for_dispatch(&mut self, event: Self::Events) {
//         unimplemented!()
//     }
// }
//
// /* Event */
// pub enum ItemEvents {
//     ItemMovedIntoShoppingCart(ItemMovedIntoShoppingCart),
// }
//
// pub enum CustomerEvents {
//     CustomerSignedIn(CustomerSignedIn),
// }
//
// pub enum OrderEvents {
//     OrderProcessed(OrderProcessed),
// }
//
// #[derive(Debug, Clone)]
// pub struct OrderProcessedPayload {
//     pub message: String
// }
//
// #[derive(Debug, Clone)]
// pub struct OrderProcessed {
//     date_time_occurred: DateTime<Utc>,
//     payload: OrderProcessedPayload
// }
//
// impl Event for OrderProcessed {
//     type DateTime = DateTime<Utc>;
//     type PublisherId = CustomerID;
//     type Payload = OrderProcessedPayload;
//
//     fn new(publisher_id: Self::PublisherId, payload: Self::Payload) -> Self {
//         OrderProcessed {
//             date_time_occurred: Utc::now(),
//             payload
//         }
//     }
//
//     fn get_date_time_occurred(&self) -> Self::DateTime {
//         self.date_time_occurred
//     }
//
//     // TODO: Should an Order aggregate emit Order events or events of its child?
//     fn get_publisher_id(&self) -> Self::PublisherId {
//         self.get_publisher_id()
//     }
//
//     fn get_payload(&self) -> Self::Payload {
//         self.payload.clone()
//     }
//
//     fn get_event_name(&self) -> &str {
//         std::any::type_name::<Self>()
//     }
//
// }
//
// #[derive(Debug, Clone)]
// pub struct CustomerSignedInPayload {
//     pub message: String
// }
//
// #[derive(Debug, Clone)]
// pub struct CustomerSignedIn {
//     date_time_occurred: DateTime<Utc>,
//     entity_id: CustomerID,
//     payload: CustomerSignedInPayload
// }
//
// impl Event for CustomerSignedIn {
//     type DateTime = DateTime<Utc>;
//     type PublisherId = CustomerID;
//     type Payload = CustomerSignedInPayload;
//
//     fn new(publisher_id: Self::PublisherId, payload: Self::Payload) -> Self {
//         CustomerSignedIn {
//             date_time_occurred: Utc::now(),
//             entity_id: publisher_id,
//             payload
//         }
//     }
//
//     fn get_date_time_occurred(&self) -> Self::DateTime {
//         self.date_time_occurred
//     }
//
//     fn get_publisher_id(&self) -> Self::PublisherId {
//         self.entity_id.clone()
//     }
//
//     fn get_payload(&self) -> Self::Payload {
//         self.payload.clone()
//     }
//
//     fn get_event_name(&self) -> &str {
//         std::any::type_name::<Self>()
//     }
//
// }
//
// #[derive(Debug, Clone)]
// pub struct ItemMovedIntoShoppingCart {
//     date_time_occurred: DateTime<Utc>,
//     entity_id: ItemID,
//     payload: String
// }
//
// impl Event for ItemMovedIntoShoppingCart {
//     type DateTime = DateTime<Utc>;
//     type PublisherId = ItemID;
//     type Payload = String;
//
//     fn new(publisher_id: Self::PublisherId, payload: Self::Payload) -> Self {
//         ItemMovedIntoShoppingCart {
//             date_time_occurred: Utc::now(),
//             entity_id: publisher_id,
//             payload: "Simple payload".to_string()
//         }
//     }
//
//     fn get_date_time_occurred(&self) -> Self::DateTime {
//         self.date_time_occurred
//     }
//
//     fn get_publisher_id(&self) -> Self::PublisherId {
//         self.entity_id.clone()
//     }
//
//     fn get_payload(&self) -> Self::Payload {
//         self.payload.clone()
//     }
//
//     fn get_event_name(&self) -> &str {
//         std::any::type_name::<Self>()
//     }
// }
//
// /* Entity Identity */
//
// #[derive(Debug, Clone, Eq, PartialEq)]
// pub struct ItemID {
//     id: Uuid,
// }
//
// impl DomainIdentity<Uuid> for ItemID {
//     fn new() -> Self {
//         let uuid = Uuid::new_v4();
//         ItemID { id: uuid }
//     }
//
//     fn new_from_existing(value: Uuid) -> Self {
//         ItemID { id: value }
//     }
//
//     fn get_id(&self) -> Uuid {
//         self.id.clone()
//     }
// }
//
// /* Child Entity */
//
// #[derive(Debug, Clone)]
// pub struct ItemProps {
//     pub price: Price,
// }
//
// pub struct Item {
//     identity: ItemID,
//     pub props: ItemProps,
// }
//
// impl Entity<ItemID, ItemProps> for Item {
//     type Events = ();
//
//     fn new(id: ItemID, props: ItemProps) -> Self {
//         Item { identity: id, props }
//     }
//
//     fn get_identity(&self) -> ItemID {
//         self.identity.clone()
//     }
//
//     fn is_same_as(&self, other: impl Entity<ItemID, ItemProps>) -> bool {
//         self.identity.id == other.get_identity().id
//     }
//
//     fn add_event_for_dispatch(&mut self, event: Self::Events) {
//         unimplemented!()
//     }
// }
//
// /* Value Object */
//
// #[derive(Debug, Eq, PartialEq, Clone)]
// pub struct PriceProps {
//     pub currency: String,
//     pub amount: i32,
//     pub scale: i8
// }
//
// #[derive(Debug, Eq, PartialEq, Clone)]
// pub struct Price {
//     props: PriceProps,
// }
//
// impl ValueObject<PriceProps> for Price {
//     fn new(props: PriceProps) -> Self {
//         Price { props }
//     }
//
//     fn get_props(&self) -> PriceProps {
//         self.props.clone()
//     }
//
//     fn equals(&self, other: impl ValueObject<PriceProps>) -> bool {
//         self.props == other.get_props()
//     }
// }
//
// /* Value Object */
//
// #[derive(Debug, Eq, PartialEq, Clone)]
// pub struct AddressProps {
//     pub address: String
// }
//
// #[derive(Debug, Eq, PartialEq, Clone)]
// pub struct Address {
//     props: AddressProps,
// }
//
// impl ValueObject<AddressProps> for Address {
//     fn new(props: AddressProps) -> Self {
//         Address { props }
//     }
//
//     fn get_props(&self) -> AddressProps {
//         self.props.clone()
//     }
//
//     fn equals(&self, other: impl ValueObject<AddressProps>) -> bool {
//         self.props == other.get_props()
//     }
// }
//
//
// fn main() {
//
//     /* Item Entity (root) */
//     let item_id = ItemID::new();
//     let item_props = ItemProps {
//         price: Price { props: PriceProps {
//             currency: "YEN".to_string(),
//             amount: 12345678,
//             scale: 2
//         } }
//     };
//     let item= Item::new(item_id, item_props);
//
//     /* Order Entity (child) */
//     let customer_id = CustomerID::new();
//     let customer_address = Address::new(AddressProps { address: "Foo".to_string() });
//     let customer_props = CustomerProps { value_object: customer_address };
//     let customer = Customer::new(customer_id, customer_props);
//
//     /* Order Aggregate */
//     let order = Order::new(customer, vec![item]);
//
// }

// TODO:
fn main() {}