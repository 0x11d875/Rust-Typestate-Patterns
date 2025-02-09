// This is an example that won't compile because we try to generate a function with an invalid transition.

extern crate typestate;

use typestate::{add_states, impl_for, just_a_comment, transition};

// The only valid transition is from Off -> On
#[add_states(
    "States: Off, On; 
    Transitions:
            Off -> On"
)]

struct SimpleMachine {
    name: String,
}

#[impl_for]
impl SimpleMachine {
    // This is okay, because Off -> On is a valid transition
    #[transition(Off -> On)]
    fn turn_on(self) {
        println!("Machine is now in state 'On'.");
    }

    // Failure! Invalid transition. Won't compile
    #[transition(On -> Off)]
    fn turn_off(self) {
        println!("Machine is now in state 'Off'.");
    }
}

fn main() {}

/*
Compiler error:
error[E0308]: mismatched types
  --> src/main.rs:16:1
   |
16 | #[impl_for]
   | ^^^^^^^^^^^ expected struct `SimpleMachine_state_Off`, found struct `SimpleMachine_state_On`
   |
   = note: expected type `SimpleMachine<SimpleMachine_state_Off>`
              found type `SimpleMachine<SimpleMachine_state_On>`

Which means: no specified transition
*/
