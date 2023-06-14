use crate::model::card::requirement::Requirement;
use crate::model::card::{Card, CardId};
use crate::model::game::mutation::Mutation;
use crate::model::resource::Resource;
use crate::model::tag::Tag;
use crate::model::tag::Tag::{Builder, Space};

pub(crate) struct CardBuilder {
    card_id: CardId,
    cost: i32,
    requirement: Option<Requirement>,
    tags: Vec<Tag>,
    victory_points: i32,
    other_mutations: Vec<Mutation>,
    event: bool,
}

impl CardBuilder {
    pub(crate) fn new() -> Self {
        Self {
            card_id: 0,
            cost: 0,
            requirement: None,
            tags: Vec::new(),
            victory_points: 0,
            other_mutations: Vec::new(),
            event: false,
        }
    }

    pub(crate) fn id(mut self, id: CardId) -> Self {
        self.card_id = id;
        self
    }

    pub(crate) fn cost(mut self, cost: i32) -> Self {
        self.cost += cost;
        self
    }

    pub(crate) fn tags(mut self, mut tags: Vec<Tag>) -> Self {
        self.tags.append(&mut tags);
        self
    }

    pub(crate) fn event(mut self) -> Self {
        self.event = true;
        self
    }

    pub(crate) fn requirement(mut self, requirement: Requirement) -> Self {
        self.requirement = Some(requirement);
        self
    }

    pub(crate) fn mutation(mut self, mutation: Mutation) -> Self {
        self.other_mutations.push(mutation);
        self
    }

    pub(crate) fn victory_points(mut self, amount: i32) -> Self {
        self.victory_points += amount;
        self
    }

    pub(crate) fn build(mut self) -> Card {
        let mut mutations = vec![
            Mutation::CardPlay(self.card_id),
            self.get_payment_mutation(),
        ];

        if self.victory_points > 0 {
            mutations.push(Mutation::VictoryPoint(self.victory_points))
        }

        if !self.event {
            mutations.extend(self.tags.iter().map(|tag| Mutation::Tag(*tag)));
        }

        mutations.append(&mut self.other_mutations);

        Card::new(Mutation::Composite(mutations), self.requirement)
    }

    fn get_payment_mutation(&self) -> Mutation {
        if self.tags.contains(&Builder) {
            Mutation::BuilderCardPayment(self.cost)
        } else if self.tags.contains(&Space) {
            Mutation::SpaceCardPayment(self.cost)
        } else {
            Mutation::Resource(Resource::MegaCredit, -self.cost)
        }
    }
}
