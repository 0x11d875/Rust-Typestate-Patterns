extern crate typestate;

use typestate::{add_states, constructor, impl_for, just_a_comment, require_state, transition};

// If your state machine has multiple states that lead to the same state you can use '|' to seperate them.
// You can also write them individualy line by line.
#[add_states(
    "States: 
        Off, On, Idle, brewed, need_cleaning; 
    Transitions:  
        Off | Idle | need_cleaning -> On, 
        On  | need_cleaning -> Idle, 
        On  -> Off, 
        need_cleaning -> Off, 
        brewed -> Off,
        On -> brewed,
        brewed -> need_cleaning"
)]

struct CoffeeMachine {
    money: u64,
}

#[impl_for]
impl CoffeeMachine {
    // This creates a constructor for two init states 'Off' and 'Idle'.
    // The compiler is (in some cases) able to infer the right type.
    // But it is also possible to use a specific one. See below in our
    // main function for more info.
    #[constructor(Off | Idle)]
    fn new() -> CoffeeMachine {
        CoffeeMachine { money: 0 }
    }
    #[transition(Off -> On)]
    fn turn_on(self) {}

    #[transition(need_cleaning -> On)]
    fn clean(self) {}

    #[transition(need_cleaning -> On)]
    fn skip_cleaning(self) {}

    #[transition(On -> Idle)]
    fn turn_idle(self) {}

    #[transition(On -> brewed)]
    fn brew(self) {}

    #[transition(brewed -> need_cleaning)]
    fn take_coffee(self) {}

    #[transition(Idle -> On)]
    fn wake_up(self) {}

    // It is possible to use multiple outgoing states seperated by '|'.
    #[transition(On | need_cleaning -> Off)]
    fn turn_off(self) {}

    // Also you can define the same function multiple times with different transitions.
    #[transition(brewed -> Off)]
    fn turn_off(self) {}

    // 'require_state' can also be used with multiple states.
    #[require_state(On | Idle)]
    fn insert_money(&mut self, amount: u64) {
        self.money += amount;
    }
}

fn main() {
    let m1 = CoffeeMachine::new();
    let m2 = CoffeeMachine::new();
    // It is also possible to specify the state directly:
    // let m = CoffeeMachine::<CoffeeMachine_state_Idle>::new();
    // The naming convention of the type is alway the same: STRUCTNAME_state_STATENAME
    // We have enforced this naming convention to mitigate name collisions.
    // But in this cases the compiler will infer the only valid state:
    // m1 must be state 'Off', because the first function we call is a transition 'Off -> On'.
    // m2 must be state 'Idle', because the first function we call is a transition 'Idle -> On'.
    let m1 = m1.turn_on();
    let m2 = m2.wake_up();
    m2.turn_off();

    let mut m1 = m1.turn_idle();

    m1.insert_money(5);
    let m1 = m1.wake_up();
    let m1 = m1.brew();
    let m1 = m1.take_coffee();
    let m1 = m1.clean();
    m1.turn_off();
}

/*
This program will generate 4 warnings:
1. method is never used: `insert_money`
    We used insert_money(), but only in state 'Idle'. If we remove the 'On' state from the 'require_state' above this warning will disappear.
2. method is never used: `skip_cleaning`
    It's ok, we did not use it.
3. + 4. warning: method is never used: `turn_off`
    Same as 1. There are 2 other transitions ('brewed' and 'need_cleaning') we don't use.
*/
