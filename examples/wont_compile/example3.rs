// This is an example that won't compile because we try to call a function from an invalid state

extern crate typestate;

use typestate::{add_states, constructor, impl_for, just_a_comment, require_state, transition};

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
            name: "Coffe Machine".to_string(),
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

    #[require_state(On)]
    fn print_name(&self) {
        println!("My name is {:?} and my state is 'On'.", self.name);
    }
}

fn main() {
    let machine = SimpleMachine::new();
    let machine = machine.turn_on();
    let machine = machine.turn_off();
    // Invalid: machine is in state 'Off' and the function 'print_name()' is only valid in state 'On'
    machine.print_name();
}

/*
Compiler error:

error[E0599]: no method named `print_name` found for type `SimpleMachine<SimpleMachine_state_Off>` in the current scope
  --> src/main.rs:49:15
   |
15 | struct SimpleMachine {name: String}
   | -------------------- method `print_name` not found for this
...
49 |       machine.print_name();
   |               ^^^^^^^^^^ method not found in `SimpleMachine<SimpleMachine_state_Off>`

*/
