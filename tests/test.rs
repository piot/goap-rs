/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/goap-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use goap_rs::prelude::*;
use log::info;
use std::fmt::{Display, Formatter};
use test_log;

#[derive(Default, Debug)]
pub struct EatAction {
    pub debug_counter: usize,
}

impl ActionTrait for EatAction {
    fn start(&mut self) {
        info!("eat is starting!");
    }

    fn update(&mut self) -> ActionStatus {
        info!("eat is updating!");
        self.debug_counter += 1;
        if self.debug_counter < 10 {
            ActionStatus::NotReady
        } else {
            ActionStatus::Done
        }
    }
}

impl Display for EatAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "eat: {}", self.debug_counter)
    }
}

#[test_log::test]
pub fn test_id() {
    let id = PropertyId::new("hello");
    assert_eq!(id.inner(), 0x248BFA47)
}

#[test_log::test]
pub fn test_eat() {
    let mut planner = Planner::new(vec![]);

    {
        let eat_pre_conditions = State::new().push("has_food", true);

        let eat_effects = State::new().push("hungry", false);
        info!("eat_effects: {:?}", eat_effects);

        let eat_action = Action::new(
            eat_pre_conditions,
            eat_effects,
            32,
            Box::new(EatAction::default()),
        )
        .with_debug_name("EAT");

        planner.push(eat_action);
    }

    {
        let keep_health_up_state = State::new().push("max_health", true);
        let healthy_goal = Goal::new(keep_health_up_state, 1).with_debug_string("healthy goal");

        let world_state = State::new().push("max_health", false);

        let mut actions = planner.find_plan(&world_state, &healthy_goal).unwrap();
        for (index, action) in actions.actions.iter_mut().enumerate() {
            info!("   action: {}: {:?}", index, action);
        }

        for (index, action) in actions.actions.iter_mut().enumerate() {
            info!("* start action: {}: {}", index, action);
            action.start();
            while action.update() == ActionStatus::NotReady {
                info!("...not ready yet");
            }
            info!("action {} done!", action);
        }
    }
}
