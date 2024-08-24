/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/goap-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use goap_rs::prelude::*;

#[derive(Default)]
pub struct EatAction {
    pub debug_counter: usize,
}

impl ActionTrait for EatAction {
    fn start(&mut self) {
        println!("eat is starting!");
    }

    fn update(&mut self) -> ActionStatus {
        println!("eat is updating!");
        self.debug_counter += 1;
        if self.debug_counter < 10 {
            ActionStatus::NotReady
        } else {
            ActionStatus::Done
        }
    }
}

#[test]
pub fn test_id() {
    let id = PropertyId::new("hello");
    assert_eq!(id.inner(), 0x248BFA47)
}

#[test]
pub fn test_eat() {
    let eat_pre_conditions = State::new().push("has_food", true);

    let eat_effects = State::new().push("hungry", false);

    let mut eat = Action::new(
        eat_pre_conditions,
        eat_effects,
        32,
        Box::new(EatAction::default()),
    );

    eat.start();

    while eat.update() == ActionStatus::NotReady {
        println!("not ready yet");
    }

    println!("done!");
}
