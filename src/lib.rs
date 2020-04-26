use std::collections::HashMap;
use std::hash::Hash;

pub struct Machine<S, E>(StateConfig<S, E>)
where
    S: Hash + Eq + Copy,
    E: Hash + Eq;

impl<S, E> Machine<S, E>
where
    S: Hash + Eq + Copy,
    E: Hash + Eq,
{
    pub fn transition(&self, current_state: State<S>, event: E) -> Option<State<S>> {
        if let Some(config) = self.0.states.get(&current_state.value) {
            if let Some(transition) = config.on.get(&event) {
                Some(State {
                    value: transition.target,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct State<S> {
    value: S,
}

pub struct StateEventHandler<S, E>
where
    S: Hash + Eq + Copy,
    E: Hash + Eq,
{
    on: HashMap<E, EventTransition<S>>,
}

impl<S, E> StateEventHandler<S, E>
where
    S: Hash + Eq + Copy,
    E: Hash + Eq,
{
    pub fn new() -> Self {
        StateEventHandler { on: HashMap::new() }
    }

    pub fn with_event(mut self, event: E, transition: EventTransition<S>) -> Self {
        self.on.insert(event, transition);
        self
    }
}

pub struct StateConfig<S, E>
where
    S: Hash + Eq + Copy,
    E: Hash + Eq,
{
    states: HashMap<S, StateEventHandler<S, E>>,
}

impl<S, E> StateConfig<S, E>
where
    S: Hash + Eq + Copy,
    E: Hash + Eq,
{
    pub fn new() -> Self {
        StateConfig {
            states: HashMap::new(),
        }
    }

    pub fn with_state(mut self, state_key: S, handler: StateEventHandler<S, E>) -> Self {
        self.states.insert(state_key, handler);
        self
    }
}

pub struct EventTransition<S> {
    target: S,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_machine() -> Machine<States, Events> {
        Machine(
            StateConfig::new()
                .with_state(
                    States::Active,
                    StateEventHandler::new().with_event(
                        Events::Toggle,
                        EventTransition {
                            target: States::Inactive,
                        },
                    ),
                )
                .with_state(
                    States::Inactive,
                    StateEventHandler::new().with_event(
                        Events::Toggle,
                        EventTransition {
                            target: States::Active,
                        },
                    ),
                ),
        )
    }

    #[test]
    fn it_transitions() {
        let machine = create_machine();
        let initial_state: State<States> = State {
            value: States::Active,
        };
        let next_state = machine.transition(initial_state, Events::Toggle);
        println!("{:?}", &next_state);

        assert!(next_state.is_some());
        assert_eq!(next_state.unwrap().value, States::Inactive);
    }

    #[test]
    fn it_does_not_transition_with_not_handled_event() {
        let machine = create_machine();
        let initial_state: State<States> = State {
            value: States::Active,
        };

        let next_state = machine.transition(initial_state, Events::NoEvent);
        assert!(next_state.is_none());
    }

    #[test]
    fn it_does_not_transition_with_invalid_state() {
        let machine = create_machine();
        let initial_state: State<States> = State {
            value: States::NoState
        };
        let next_state = machine.transition(initial_state, Events::Toggle);
        assert!(next_state.is_none());
    }

    #[derive(Hash, PartialEq, Clone, Copy, Eq, Debug)]
    enum States {
        Active,
        Inactive,
        NoState,
    }

    #[derive(Hash, PartialEq, Clone, Copy, Eq, Debug)]
    enum Events {
        Toggle,
        NoEvent,
    }
}
