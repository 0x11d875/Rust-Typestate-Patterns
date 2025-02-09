#[just_a_comment("Creating all given states.")]
trait CoffeeMachineStates {}
#[derive(Clone)]
struct CoffeeMachine_state_Off;
impl CoffeeMachineStates for CoffeeMachine_state_Off {}
#[derive(Clone)]
struct CoffeeMachine_state_On;
impl CoffeeMachineStates for CoffeeMachine_state_On {}
#[derive(Clone)]
struct CoffeeMachine_state_Idle;
impl CoffeeMachineStates for CoffeeMachine_state_Idle {}
#[derive(Clone)]
struct CoffeeMachine_state_brewed;
impl CoffeeMachineStates for CoffeeMachine_state_brewed {}
#[derive(Clone)]
struct CoffeeMachine_state_need_cleaning;
impl CoffeeMachineStates for CoffeeMachine_state_need_cleaning {}
#[just_a_comment("Here is your given struct as generic which accepts all given states.")]
#[derive(Clone)]
struct CoffeeMachine<CoffeeMachine_mode: CoffeeMachineStates> {
    money: u64,
    typestate_mode: CoffeeMachine_mode,
}
#[just_a_comment("Impl state machine builder for every possible transition")]
trait CoffeeMachine_StateConverter<T: CoffeeMachineStates> {
    fn contruct_with_state(self) -> CoffeeMachine<T>;
}
impl CoffeeMachine_StateConverter<CoffeeMachine_state_On>
    for CoffeeMachine<CoffeeMachine_state_Off>
{
    fn contruct_with_state(self) -> CoffeeMachine<CoffeeMachine_state_On> {
        CoffeeMachine {
            money: self.money,
            typestate_mode: CoffeeMachine_state_On,
        }
    }
}
impl CoffeeMachine_StateConverter<CoffeeMachine_state_On>
    for CoffeeMachine<CoffeeMachine_state_Idle>
{
    fn contruct_with_state(self) -> CoffeeMachine<CoffeeMachine_state_On> {
        CoffeeMachine {
            money: self.money,
            typestate_mode: CoffeeMachine_state_On,
        }
    }
}
impl CoffeeMachine_StateConverter<CoffeeMachine_state_On>
    for CoffeeMachine<CoffeeMachine_state_need_cleaning>
{
    fn contruct_with_state(self) -> CoffeeMachine<CoffeeMachine_state_On> {
        CoffeeMachine {
            money: self.money,
            typestate_mode: CoffeeMachine_state_On,
        }
    }
}
impl CoffeeMachine_StateConverter<CoffeeMachine_state_Idle>
    for CoffeeMachine<CoffeeMachine_state_On>
{
    fn contruct_with_state(self) -> CoffeeMachine<CoffeeMachine_state_Idle> {
        CoffeeMachine {
            money: self.money,
            typestate_mode: CoffeeMachine_state_Idle,
        }
    }
}
impl CoffeeMachine_StateConverter<CoffeeMachine_state_Idle>
    for CoffeeMachine<CoffeeMachine_state_need_cleaning>
{
    fn contruct_with_state(self) -> CoffeeMachine<CoffeeMachine_state_Idle> {
        CoffeeMachine {
            money: self.money,
            typestate_mode: CoffeeMachine_state_Idle,
        }
    }
}
impl CoffeeMachine_StateConverter<CoffeeMachine_state_Off>
    for CoffeeMachine<CoffeeMachine_state_On>
{
    fn contruct_with_state(self) -> CoffeeMachine<CoffeeMachine_state_Off> {
        CoffeeMachine {
            money: self.money,
            typestate_mode: CoffeeMachine_state_Off,
        }
    }
}
impl CoffeeMachine_StateConverter<CoffeeMachine_state_Off>
    for CoffeeMachine<CoffeeMachine_state_need_cleaning>
{
    fn contruct_with_state(self) -> CoffeeMachine<CoffeeMachine_state_Off> {
        CoffeeMachine {
            money: self.money,
            typestate_mode: CoffeeMachine_state_Off,
        }
    }
}
impl CoffeeMachine_StateConverter<CoffeeMachine_state_Off>
    for CoffeeMachine<CoffeeMachine_state_brewed>
{
    fn contruct_with_state(self) -> CoffeeMachine<CoffeeMachine_state_Off> {
        CoffeeMachine {
            money: self.money,
            typestate_mode: CoffeeMachine_state_Off,
        }
    }
}
impl CoffeeMachine_StateConverter<CoffeeMachine_state_brewed>
    for CoffeeMachine<CoffeeMachine_state_On>
{
    fn contruct_with_state(self) -> CoffeeMachine<CoffeeMachine_state_brewed> {
        CoffeeMachine {
            money: self.money,
            typestate_mode: CoffeeMachine_state_brewed,
        }
    }
}
impl CoffeeMachine_StateConverter<CoffeeMachine_state_need_cleaning>
    for CoffeeMachine<CoffeeMachine_state_brewed>
{
    fn contruct_with_state(self) -> CoffeeMachine<CoffeeMachine_state_need_cleaning> {
        CoffeeMachine {
            money: self.money,
            typestate_mode: CoffeeMachine_state_need_cleaning,
        }
    }
}
#[just_a_comment("Creating trait for needed new() function.")]
trait CoffeeMachine_InitialConstructor<T: CoffeeMachineStates> {
    fn new() -> CoffeeMachine<T>;
}
#[just_a_comment("Impl new() for state: Off ")]
impl CoffeeMachine_InitialConstructor<CoffeeMachine_state_Off>
    for CoffeeMachine<CoffeeMachine_state_Off>
{
    # [constructor (Off | Idle)]
    fn new() -> CoffeeMachine<CoffeeMachine_state_Off> {
        CoffeeMachine {
            money: 0,
            typestate_mode: CoffeeMachine_state_Off,
        }
    }
}
#[just_a_comment("Impl new() for state: Idle ")]
impl CoffeeMachine_InitialConstructor<CoffeeMachine_state_Idle>
    for CoffeeMachine<CoffeeMachine_state_Idle>
{
    # [constructor (Off | Idle)]
    fn new() -> CoffeeMachine<CoffeeMachine_state_Idle> {
        CoffeeMachine {
            money: 0,
            typestate_mode: CoffeeMachine_state_Idle,
        }
    }
}
#[just_a_comment("Impl all functions for state: CoffeeMachine_state_Idle")]
impl CoffeeMachine<CoffeeMachine_state_Idle> {
    # [transition (Idle -> On)]
    fn wake_up(self) -> CoffeeMachine<CoffeeMachine_state_On> {
        CoffeeMachine::contruct_with_state(self)
    }
    #
    [require_state (On | Idle)]
    fn insert_money(&mut self, amount: u64) {
        self.money += amount;
    }
}
#[just_a_comment("Impl all functions for state: CoffeeMachine_state_Off")]
impl CoffeeMachine<CoffeeMachine_state_Off> {
    # [transition (Off -> On)]
    fn turn_on(self) -> CoffeeMachine<CoffeeMachine_state_On> {
        CoffeeMachine::contruct_with_state(self)
    }
}
#[just_a_comment("Impl all functions for state: CoffeeMachine_state_On")]
impl CoffeeMachine<CoffeeMachine_state_On> {
    # [transition (On -> Idle)]
    fn turn_idle(self) -> CoffeeMachine<CoffeeMachine_state_Idle> {
        CoffeeMachine::contruct_with_state(self)
    }
    # [transition (On -> brewed)]
    fn brew(self) -> CoffeeMachine<CoffeeMachine_state_brewed> {
        CoffeeMachine::contruct_with_state(self)
    }
    #
    [transition (On | need_cleaning -> Off)]
    fn turn_off(self) -> CoffeeMachine<CoffeeMachine_state_Off> {
        CoffeeMachine::contruct_with_state(self)
    }
    #
    [require_state (On | Idle)]
    fn insert_money(&mut self, amount: u64) {
        self.money += amount;
    }
}
#[just_a_comment("Impl all functions for state: CoffeeMachine_state_brewed")]
impl CoffeeMachine<CoffeeMachine_state_brewed> {
    # [transition (brewed -> need_cleaning)]
    fn take_coffee(self) -> CoffeeMachine<CoffeeMachine_state_need_cleaning> {
        CoffeeMachine::contruct_with_state(self)
    }
    #
    [transition (brewed -> Off)]
    fn turn_off(self) -> CoffeeMachine<CoffeeMachine_state_Off> {
        CoffeeMachine::contruct_with_state(self)
    }
}
#[just_a_comment("Impl all functions for state: CoffeeMachine_state_need_cleaning")]
impl CoffeeMachine<CoffeeMachine_state_need_cleaning> {
    # [transition (need_cleaning -> On)]
    fn clean(self) -> CoffeeMachine<CoffeeMachine_state_On> {
        CoffeeMachine::contruct_with_state(self)
    }
    #
    [transition (need_cleaning -> On)]
    fn skip_cleaning(self) -> CoffeeMachine<CoffeeMachine_state_On> {
        CoffeeMachine::contruct_with_state(self)
    }
    #
    [transition (On | need_cleaning -> Off)]
    fn turn_off(self) -> CoffeeMachine<CoffeeMachine_state_Off> {
        CoffeeMachine::contruct_with_state(self)
    }
}
