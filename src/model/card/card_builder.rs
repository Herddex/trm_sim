use crate::model::card::Card;
use crate::model::game_data::mutation::Mutation;
use crate::model::game_data::mutation::NormalMutation;
use crate::model::game_data::mutation::NormalMutation::{ResourceMutation, TagMutation, VictoryPointMutation};
use crate::model::game_data::requirement::NewRequirement;
use crate::model::resource::Resource;
use crate::model::tag::Tag;

pub struct CardBuilder {
    cost: i32,
    requirement: Option<NewRequirement>,
    tags: Vec<Tag>,
    victory_points: i32,
    mutations: Vec<NormalMutation>,
    event: bool
}

impl CardBuilder {
    pub fn new() -> Self {
        Self {
            cost: 0,
            requirement: None,
            tags: Vec::new(),
            victory_points: 0,
            mutations: Vec::new(),
            event: false
        }
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

    pub fn requirement(mut self, requirement: NewRequirement) -> Self {
        self.requirement = Some(requirement);
        self
    }

    pub fn mutation(mut self, mutation: NormalMutation) -> Self {
        self.mutations.push(mutation);
        self
    }

    pub fn victory_points(mut self, amount: i32) -> Self {
        self.victory_points += amount;
        self
    }

    pub fn build(mut self) -> Card {
        let mut mutations= Vec::new();
        if self.cost > 0 {
            mutations.push(ResourceMutation(Resource::MegaCredit, self.cost));
        }
        if self.victory_points > 0 {
            mutations.push(VictoryPointMutation(self.victory_points))
        }
        if !self.event {
            for tag in self.tags {
                mutations.push(TagMutation(tag))
            }
        }
        mutations.append(&mut self.mutations);

        Card {
            requirement: self.requirement,
            immediate_effects: Mutation::Normal(NormalMutation::CompositeMutation(mutations)),
        }
    }
}