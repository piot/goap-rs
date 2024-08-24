/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/goap-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use mash_rs::murmur3_32;
use std::collections::HashSet;
pub mod prelude;

pub struct PropertyId(u32);

impl PropertyId {
    pub fn new(name: &str) -> Self {
        Self(murmur3_32(name.as_bytes(), 0))
    }

    pub fn inner(self) -> u32 {
        self.0
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct ConditionId {
    id: u64,
    debug_str: String,
}

impl ConditionId {
    pub fn new(name: &str, value: bool) -> Self {
        let property = PropertyId::new(name);
        let calculated_value = property.0 << 1 | if value { 1 } else { 0 };

        Self {
            id: calculated_value as u64,
            debug_str: Default::default(),
        }
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

type Cost = u32;

#[derive(PartialEq)]
pub enum ActionStatus {
    Done,
    NotReady,
    Cancelled,
}

pub trait ActionTrait {
    fn start(&mut self);
    fn update(&mut self) -> ActionStatus;
}

#[allow(unused)]
pub struct Action {
    pre_conditions: State,
    effects: State,
    cost: u32,
    debug_name: String,
    implementation: Box<dyn ActionTrait>,
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
