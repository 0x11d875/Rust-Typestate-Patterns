# Rust typestate patterns

# abstract
Using state machines provides strong guarantees. Only valid transitions bet-
ween defined states are possible. Using them during compilation guarantees
that incorrect control-flow will not appear in binaries. Furthermore it can
help to improve performance. Especially for embedded devices and critical
applications runnning in weak hardware environments runtime checks are
quit a big loss of performance. Using a state machine while compiling will
eliminate such problems. We used rust’s type-system and ownership model
to build a macro library to support easier usage of state machines in coding
projects.


# Rust typestate patterns
Rust typestate patterns (short: RTP) will guarantee your specified control-flow without
any overhead past the initial compilation. It is therefore especially suited for embedded
devices and critical applications runnning in weak hardware environments. RTP saves
you a lot of runtime checks, enforces your defined control flow and makes analyzing and
testing your code easier. It also helps you to visualize and prove your control flow by
generating .svg graphics of your underlying state machine and prints all generated code
into a debug file for easier maintenance, debugging and error correction.



# Macros
The library defines a list of macros to support an easier use of state machines. In most
cases it is enough to add a macro with specific parameters above the defined struct and
its implementation. For details please have a look at each macro’s description below.
We used the crate syn to build and edit the rust syntax tree because we need to generate
many different structs, traits and functions or had to modify them (e.g. add/change the
return type, add parameters etc.). For that purpose we also used the crate quote to
create proc macro::TokenStreams out of the syntax tree. Building and editing the syn
trees is often very hard and cumbersome. Therefore we used a little trick: we wrote some
dummy functions as strings and parsed them via parse macro input!. We were then able
to extract specific parts out of their syntax trees to incorporate them into our newly
generated ones.
Our macros are easy to use and will generate all of the additional code required which
then is incorporated during compilation, e.g. for each state the library creates a struct
and its implementations for all generated traits. Therefore the code stays easily reada-
ble during actual development without the bloated and complex structure of a manual
implementation of type states.




## add states
This macro reads and parses the given state machine and creates all required states as
structs. It will also add a type to the given struct. This type can only be from the set of
all defined states. Thereafter all possible transitions are generated via Rust traits. It’s
basically a “state converter“which creates a new object out of the given one. All existing
values are faithfully recreated and the associated state is changed.

The code

	#[add_states(
	"States:
		Off, On;
	Transitions:
		Off -> On,
		On -> Off"
	)]



will add two states to the struct SimpleMachine. Furthermore there now exist two valid
transitions: Off → On and On → Off. ORing states is also possible:


	#[add_states(
	"States:
		Off, On, Idle;
	Transitions:
		Off -> On,
		On | Idle -> Off"
	)]
	struct SimpleMachine {}




The macro creates for each given state a struct and implements it for a generated trait.
The trait is then used in the provided struct as a typestate. This guarantees that only
objects in defined states can exist.
The macro also generates a state converter for each defined transition. The implementa-
tion of this trait is for each state and contains a copy-constructor which uses all provided
attributes of the object and solely changes the state. The non-existance of invalid states
provides guarantees that it is not possible to use undefined transitions without errors
during compilation.


## impl_for

This macro is the root function for all code generation. It use the transition, require state
macro and constructor to decide what is to do and which corresponding functions should
be called. In the end all states are looped and for each state a specific impl for is gene-
rated. In this impl all functions that are only valid in the specific state are inserted.



### constructor

This macro is inserted just above a constructor for a struct. Without it the library can-
not create constructors for the initial states because it does not know how to construct
a specific struct (what are the initial values? what else needs to be done?).


The macro generates for each defined initial state a new constructor.


### require_state

This macro takes care that the object is in the right state. It supports single and also
multiple states as arguments.

	#[require_state(On)]
	fn dummy(&self) {
	}


 guarantees that the function dummy is only callable from state On.

 	#[require_state(On | Idle)]
	fn dummy(&self) {
	}

 guarantees that the function dummy is only callable from state On or state Idle-


 ### transition

 This macro guarantees that a function is only callable from a defined state. Furthermore
the object changes its state after the transition.

	#[transition(On -> Off)]
	fn turn_off(self) {
	}

This function is only callable from state On. After the function call the object is in the state Off.
ORing is also possible:

	#[transition(On | Idle -> Off)]
	fn turn_off(self) {
	}
 
This function is only callable from state On or state Idle. After the function call the object is in state Off.
ORing is useful because you don't need to write multiple functions that basically do the same.\\


The macro generates a state converter for each transition. The implementation of this trait is for each state and contains a copy-constructor which uses all provided attributes of the object and solely changes the state. The non-existance of invalid states provides guarantees that it is not possible to use undefined transitions without errors during compilation.


# Debugging

The lib will create a folder ’debug’ in the root folder of your project with the following
content:
1. STRUCTNAME generated states.rs
2. STRUCTNAME possible transitions.gv
3. STRUCTNAME used transitions.gv
The first file contains the code that was generated by the library. You can format it with
’rustfmt’ for easier reading. It will help you a lot if you want to see how your code is
translated.
With 2. and 3. you can visualize your statemachine as a graph. For that you first need
to install the package graphviz. Then you can execute the following command

    dot −Tsvg INPUT −o OUT. svg 


STRUCTNAME possible transitions.gv contains a graph with all possible transitions.
These are all specified transitions in add states.
STRUCTNAME used transitions.gv only contains transitions for which a function in
your code exists.





# Example
  
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
        println!("My name is {:?} and my state is 'On'.", self.name);
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
    













