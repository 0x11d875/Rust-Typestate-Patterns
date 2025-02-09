extern crate typestate;

use typestate::{add_states, constructor, impl_for, just_a_comment, require_state, transition};

#[add_states(
    "States: 
        Off, On; 
    Transitions:
        Off -> On, 
        On -> Off"
)]

struct SimpleMachine1 {name: String}


#[impl_for]
impl SimpleMachine1 {

    #[constructor(Off)]
    fn new(new_name: String) -> SimpleMachine1 {
        println!("SimpleMachine1: Creating a new machine. The init state is 'Off'.");
        SimpleMachine1 {name: new_name}
    }

    // You have to specify which transition is performed by your function.
    #[transition(Off -> On)]
    fn turn_on(self) {
        println!("SimpleMachine1 is now in state 'On'.");
    }

    #[transition(On -> Off)]
    fn turn_off(self) {
        println!("SimpleMachine1 is now in state 'Off'.");
    }

    #[require_state(On)]
    fn print_name(&self) {
        println!("SimpleMachine1: My name is {:?} and my state is 'On'.", self.name );
    }
}


#[add_states(
    "States: 
        Off, On; 
    Transitions:
        Off -> On, 
        On -> Off"
)]

struct SimpleMachine1 {name: String}


#[impl_for]
impl SimpleMachine2 {

    #[constructor(Off)]
    fn new(new_name: String) -> SimpleMachine2 {
        println!("SimpleMachine2: Creating a new machine. The init state is 'Off'.");
        SimpleMachine2 {name: new_name}
    }

    // You have to specify which transition is performed by your function.
    #[transition(Off -> On)]
    fn turn_on(self) {
        println!("SimpleMachine2 is now in state 'On'.");
    }

    #[transition(On -> Off)]
    fn turn_off(self) {
        println!("SimpleMachine2 is now in state 'Off'.");
    }

    #[require_state(On)]
    fn print_name(&self) {
        println!("SimpleMachine2: My name is {:?} and my state is 'On'.", self.name );
    }
}


fn main() {
    let machine1 = SimpleMachine1::new("Coffe Machine".to_string());
    let machine1 = machine1.turn_on();
    machine1.print_name();
    machine1.turn_off();

    let machine2 = SimpleMachine2::new("Snack Machine".to_string());
    let machine2 = machine2.turn_on();
    machine2.print_name();
    machine2.turn_off();
}
