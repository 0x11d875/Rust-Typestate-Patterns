#[just_a_comment("Creating all given states.")]
trait SimpleMachineStates {}
#[derive(Clone)]
struct SimpleMachine_state_Off;
impl SimpleMachineStates for SimpleMachine_state_Off {}
#[derive(Clone)]
struct SimpleMachine_state_On;
impl SimpleMachineStates for SimpleMachine_state_On {}
#[just_a_comment("Here is your given struct as generic which accepts all given states.")]
#[derive(Clone)]
struct SimpleMachine<S: SimpleMachineStates> {
    name: String,
    mode: S,
}
#[just_a_comment("Impl state machine builder for every possible transition")]
trait SimpleMachine_StateConverter<T: SimpleMachineStates> {
    fn contruct_with_state(self) -> SimpleMachine<T>;
}
impl SimpleMachine_StateConverter<SimpleMachine_state_On>
    for SimpleMachine<SimpleMachine_state_Off>
{
    fn contruct_with_state(self) -> SimpleMachine<SimpleMachine_state_On> {
        SimpleMachine {
            name: self.name,
            mode: SimpleMachine_state_On,
        }
    }
}
impl SimpleMachine_StateConverter<SimpleMachine_state_Off>
    for SimpleMachine<SimpleMachine_state_On>
{
    fn contruct_with_state(self) -> SimpleMachine<SimpleMachine_state_Off> {
        SimpleMachine {
            name: self.name,
            mode: SimpleMachine_state_Off,
        }
    }
}
#[just_a_comment("Creating trait for needed new() function.")]
trait SimpleMachine_InitialConstructor<T: SimpleMachineStates> {
    fn new() -> SimpleMachine<T>;
}

///////////////////////////////////////////////////////

#[just_a_comment("Impl new() for state: Off ")]
impl SimpleMachine_InitialConstructor<SimpleMachine_state_Off>
    for SimpleMachine<SimpleMachine_state_Off>
{
    #[constructor(Off)]
    fn new() -> SimpleMachine<SimpleMachine_state_Off> {
        println!("Creating a new machine. The init state is 'Off'");
        SimpleMachine {
            name: "test123".to_string(),
            mode: SimpleMachine_state_Off,
        }
    }
}
#[just_a_comment("Impl all functions for state: SimpleMachine_state_Off")]
impl SimpleMachine<SimpleMachine_state_Off> {
    # [transition (Off -> On)]
    fn turn_on(self) -> SimpleMachine<SimpleMachine_state_On> {
        println!("Machine is now in state 'On'");
        SimpleMachine::contruct_with_state(self)
    }
}
#[just_a_comment("Impl all functions for state: SimpleMachine_state_On")]
impl SimpleMachine<SimpleMachine_state_On> {
    # [transition (On -> Off)]
    fn turn_off(self) -> SimpleMachine<SimpleMachine_state_Off> {
        println!("Machine is now in state 'Off'");
        SimpleMachine::contruct_with_state(self)
    }
    #[require_state(On)]
    fn print_name(self) {
        println!("My name is {:?} and my state is 'On'.", self.name);
    }
}

///////////////////////////////////////////////////////
