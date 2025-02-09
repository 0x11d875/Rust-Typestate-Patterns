// This is an example that won't compile because we try to call a function from an invalid state

extern crate typestate;

use typestate::{add_states, constructor, impl_for, just_a_comment, transition};

// The only valid transition is from Off -> On
#[add_states(
    "States: Off, On; 
    Transitions:
            Off -> On,
            On -> Off"
)]

struct SimpleMachine {
    name: String,
}

#[impl_for]
impl SimpleMachine {
    #[constructor(Off)]
    fn new() -> SimpleMachine {
        println!("Creating a new machine. The init state is 'Off'.");
        SimpleMachine {
            name: "test123".to_string(),
        }
    }

    // This is okay, because Off -> On is a valid transition
    #[transition(Off -> On)]
    fn turn_on(self) {
        println!("Machine is now in state 'On'.");
    }

    // This is okay, because On -> Off is a valid transition
    #[transition(On -> Off)]
    fn turn_off(self) {
        println!("Machine is now in state 'Off'.");
    }
}

fn main() {
    let machine = SimpleMachine::new();
    let machine = machine.turn_on();
    // Invalid: machine is already in state 'On' and the transition 'On' -> 'On' is invalid.
    let machine = machine.turn_on();
}

/*
Compiler error:
error[E0599]: no method named `turn_on` found for type `SimpleMachine<SimpleMachine_state_On>` in the current scope
  --> src/main.rs:44:29
   |
15 | struct SimpleMachine {name: String}
   | -------------------- method `turn_on` not found for this
...
44 |       let machine = machine.turn_on();

Which means: no specified transition
*/
