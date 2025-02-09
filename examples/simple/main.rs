extern crate typestate;

use typestate::{add_states, constructor, impl_for, just_a_comment, require_state, transition};

// First you need to specify which states and which transitions exist.
// You need to add the 'add_states' macro above your struct.
#[add_states(
    "States: 
        Off, On; 
    Transitions:
        Off -> On, 
        On -> Off"
)]

struct SimpleMachine {name: String}

// Then you can write your implementation for the struct, but you need to add the 'impl_for' macro.
// It allows the library to encapsulate all the information it needs.
#[impl_for]
impl SimpleMachine {

    // You need to specify a constructor because the library needs it to init a new object.
    // This is akin to an initial state.
    #[constructor(Off)]
    fn new(new_name: String) -> SimpleMachine {
        println!("Creating a new machine. The init state is 'Off'.");
        SimpleMachine {name: new_name}
    }

    // You have to specify which transition is performed by your function.
    #[transition(Off -> On)]
    fn turn_on(self) {
        println!("Machine is now in state 'On'.");
    }

    #[transition(On -> Off)]
    fn turn_off(self) {
        println!("Machine is now in state 'Off'.");
    }

    // If you don't want to perform a transition but your function should only be executed in a
    // specific set of states use the 'require_state' macro.
    #[require_state(On)]
    fn print_name(&self) {
        println!("My name is {:?} and my state is 'On'.", self.name );
    }
}

fn main() {
    let machine = SimpleMachine::new("Coffe Machine".to_string());
    let machine = machine.turn_on();
    machine.print_name();
    machine.turn_off();
}

/*
The output of this program is: 

Creating a new machine. The init state is 'Off'.
Machine is now in state 'On'.
My name is "Coffe Machine" and my state is 'On'.
Machine is now in state 'Off'.

*/
