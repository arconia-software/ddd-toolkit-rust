use uuid::Uuid;
use ddd_toolkit::building_blocks::domain::entity::{Entity, EventDispatchable};
use ddd_toolkit::building_blocks::domain::identity::DomainIdentity;
use ddd_toolkit::building_blocks::domain::value_object::ValueObject;

/* Entity Identity */

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UserId {
    id: Uuid,
}

impl DomainIdentity<Uuid> for UserId {
    fn new() -> Self {
        let uuid = Uuid::new_v4();
        UserId { id: uuid }
    }

    fn new_from_existing(value: Uuid) -> Self {
        UserId { id: value }
    }

    fn get_id(&self) -> Uuid {
        self.id.clone()
    }
}

/* Entity */

#[derive(Debug, Copy, Clone)]
pub struct UserProps {
    pub score: Score,
}

pub struct User {
    identity: UserId,
    pub props: UserProps,
}

impl Entity<UserId, UserProps> for User {
    fn new(id: UserId, props: UserProps) -> Self {
        User { identity: id, props }
    }

    fn get_identity(&self) -> UserId {
        self.identity.clone()
    }

    fn is_same_as(&self, other: impl Entity<UserId, UserProps>) -> bool {
        self.identity.id == other.get_identity().id
    }

}

impl EventDispatchable for User {
    type Events = ();

    // TODO:
    fn add_event_for_dispatch(&mut self, _event: Self::Events) {
        unimplemented!()
    }
}

/* Value Object */

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct ScoreProps {
    pub rating: i32,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Score {
    props: ScoreProps,
}

impl ValueObject<ScoreProps> for Score {
    fn new(props: ScoreProps) -> Self {
        Score { props }
    }

    fn get_props(&self) -> ScoreProps {
        self.props
    }

    fn equals(&self, other: impl ValueObject<ScoreProps>) -> bool {
        self.props == other.get_props()
    }
}

/* Client Example */

fn main() {
    let score = Score::new(ScoreProps { rating: 11 });
    let id = UserId::new();
    let user = User::new(id, UserProps { score });
    assert_eq!(user.props.score.props.rating, 11)
}