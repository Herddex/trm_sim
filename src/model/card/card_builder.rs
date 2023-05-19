use crate::model::card::{Card, CardId};
use crate::model::game::mutation::Mutation;
use crate::model::card::requirement::Requirement;
use crate::model::resource::Resource;
use crate::model::tag::Tag;
use crate::model::tag::Tag::{Builder, Space};

pub struct CardBuilder {
    id: CardId,
    cost: i32,
    requirement: Option<Requirement>,
    tags: Vec<Tag>,
    victory_points: i32,
    mutations: Vec<Mutation>,
    event: bool,
}

impl CardBuilder {
    pub fn new() -> Self {
        Self {
            id: 0,
            cost: 0,
            requirement: None,
            tags: Vec::new(),
            victory_points: 0,
            mutations: Vec::new(),
            event: false,
        }
    }

    pub fn id(mut self, id: CardId) -> Self {
        self.id = id;
        self
    }

    pub fn cost(mut self, cost: i32) -> Self {
        self.cost += cost;
        self
    }

    pub fn tags(mut self, mut tags: Vec<Tag>) -> Self {
        self.tags.append(&mut tags);
        self
    }

    pub fn event(mut self) -> Self {
        self.event = true;
        self
    }

    pub fn requirement(mut self, requirement: Requirement) -> Self {
        self.requirement = Some(requirement);
        self
    }

    pub fn mutation(mut self, mutation: Mutation) -> Self {
        self.mutations.push(mutation);
        self
    }

    pub fn victory_points(mut self, amount: i32) -> Self {
        self.victory_points += amount;
        self
    }

    pub fn build(mut self) -> Card {
        let mut mutations = Vec::new();
        mutations.push(if self.tags.contains(&Builder) {
            Mutation::BuilderCardPayment(self.cost)
        } else if self.tags.contains(&Space) {
            Mutation::SpaceCardPayment(self.cost)
        } else {
            Mutation::Resource(Resource::MegaCredit, -self.cost)
        });

        if self.victory_points > 0 {
            mutations.push(Mutation::VictoryPoint(self.victory_points))
        }
        if !self.event {
            for tag in self.tags {
                mutations.push(Mutation::Tag(tag))
            }
        }
        mutations.append(&mut self.mutations);

        Card::new(self.id, mutations, self.requirement)
    }
}
