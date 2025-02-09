// This is an example that won't compile because we try to use a state that does not exist.

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
// Invalid use of a non-existing state
impl SimpleMachine {
    #[require_state(nonExisting)]
    fn function(self) {}
}

fn main() {}

/*
Compiler error:
error: cannot find attribute `require_state` in this scope
  --> src/main.rs:20:7
   |
20 |     #[require_state(nonExisting)]
   |       ^^^^^^^^^^^^^

error[E0412]: cannot find type `SimpleMachine_state_nonExisting` in this scope
  --> src/main.rs:18:1

*/
