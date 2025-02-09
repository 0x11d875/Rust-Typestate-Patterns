// This is an example that won't compile because we try to specify a transition to a non-existing state.

extern crate typestate;

use typestate::{add_states, impl_for, just_a_comment, transition};

// The state 'On' does not exist because it is not listed under States.
#[add_states(
    "States: Off; 
    Transitions:
            Off -> On"
)]

struct SimpleMachine {
    name: String,
}

fn main() {}

/*
Compiler error:
error[E0412]: cannot find type `SimpleMachine_state_On` in this scope
  --> src/main.rs:8:1
   |
8  | / #[add_states(
9  | |     "States: Off;
10 | |     Transitions:
11 | |             Off -> On"
12 | | )]
   | |__^ help: a struct with a similar name exists: `SimpleMachine_state_Off`


*/
