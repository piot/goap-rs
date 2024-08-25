/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/goap-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
extern crate core;

use log::info;
use mash_rs::murmur3_32;
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};

pub mod prelude;

pub struct PropertyId(u32);

impl PropertyId {
    pub fn new(name: &str) -> Self {
        Self(murmur3_32(name.as_bytes(), 0))
    }

    pub const fn inner(self) -> u32 {
        self.0
    }
}

#[derive(Clone)]
pub struct ConditionId {
    id: u64,
    debug_str: String,
}

impl Display for ConditionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.debug_str, self.id)
    }
}

impl PartialEq<Self> for ConditionId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ConditionId {}

impl Hash for ConditionId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl ConditionId {
    pub fn new(name: &str, value: bool) -> Self {
        let property = PropertyId::new(name);
        let calculated_value = property.0 << 1 | if value { 1 } else { 0 };

        Self {
            id: calculated_value as u64,
            debug_str: format!("{}={}", name, value),
        }
    }
}

impl Debug for ConditionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({:04X})", self.debug_str, self.id)
    }
}

#[derive(Default)]
pub struct State(HashSet<ConditionId>);

impl State {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn push(mut self, name: &str, value: bool) -> Self {
        self.0.insert(ConditionId::new(name, value));
        self
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (index, condition_id) in self.0.iter().enumerate() {
            write!(f, "\n  {} {:?}", index, condition_id)?
        }
        Ok(())
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (index, condition_id) in self.0.iter().enumerate() {
            write!(f, "\n  {} {}", index, condition_id)?
        }
        Ok(())
    }
}

type Cost = u32;

#[derive(Debug, PartialEq)]
pub enum ActionStatus {
    Done,
    NotReady,
    Cancelled,
}

pub trait ActionTrait: Debug + Display {
    fn start(&mut self);
    fn update(&mut self) -> ActionStatus;
}

#[allow(unused)]
#[derive(Debug)]
pub struct Goal {
    requirements: State,
    priority: u32,
    debug_name: String,
}

impl Display for Goal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "goal:{}", self.debug_name)
    }
}

impl Goal {
    pub fn new(requirements: State, priority: u32) -> Self {
        Self {
            requirements,
            priority,
            debug_name: Default::default(),
        }
    }

    pub fn with_debug_string(mut self, s: &str) -> Self {
        self.debug_name = s.to_string();
        self
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct Action {
    pre_conditions: State,
    effects: State,
    cost: u32,
    debug_name: String,
    implementation: Box<dyn ActionTrait>,
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "'{}' {} {}",
            self.debug_name, self.pre_conditions, self.effects
        )
    }
}

impl Action {
    pub fn new(
        pre_conditions: State,
        effects: State,
        cost: Cost,
        implementation: Box<dyn ActionTrait>,
    ) -> Self {
        Self {
            pre_conditions,
            effects,
            cost,
            debug_name: Default::default(),
            implementation,
        }
    }

    pub fn with_debug_name(mut self, name: &str) -> Self {
        self.debug_name = name.to_string();
        self
    }

    pub fn start(&mut self) {
        self.implementation.start()
    }

    pub fn update(&mut self) -> ActionStatus {
        self.implementation.update()
    }
}

pub struct Plan {
    pub actions: Vec<Action>,
}

impl Plan {
    pub fn new(actions: Vec<Action>) -> Self {
        Self { actions }
    }

    pub fn push(&mut self, action: Action) -> &mut Self {
        self.actions.push(action);
        self
    }
}

pub struct Planner {
    actions: Vec<Action>,
}

impl Planner {
    pub fn new(actions: Vec<Action>) -> Self {
        Self { actions }
    }

    pub fn push(&mut self, action: Action) -> &mut Self {
        self.actions.push(action);
        self
    }

    pub fn find_plan(self, world_state: &State, goal: &Goal) -> Option<Plan> {
        info!(
            "finding a plan for the goal {} with world state {}",
            goal, world_state
        );
        Some(Plan::new(self.actions))
    }
}
