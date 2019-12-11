use std::cell::RefCell;
use std::option::Option;
use std::rc::Rc;

struct State {
    state_num: i32,
}

struct Store {
    state: Rc<State>,
}

impl Store {
    fn new(state_num: i32) -> Store {
        let state = State { state_num };
        let state = Rc::new(state);
        Store { state }
    }

    fn update_state(&mut self, state_num: i32) {
        let new_state = State { state_num };
        let new_state = Rc::new(new_state);
        self.state = new_state;
    }

    fn get_state(&self) -> Rc<State> {
        return Rc::clone(&self.state);
    }
}

struct Reducer {
    store: Option<Rc<RefCell<Store>>>,
}

impl Reducer {
    fn new() -> Reducer {
        Reducer { store: None }
    }

    fn reduce(&mut self, event: &Event) {
        if let Some(store) = self.store.as_mut() {
            store.borrow_mut().update_state(event.new_state_num);
        } else {
            println!("state is None");
        }
    }

    fn register_store(&mut self, store: Rc<RefCell<Store>>) {
        self.store = Some(store);
    }
    fn unregister_store(&mut self) {
        self.store = None;
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    new_state_num: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_works() {
        let store = Rc::new(RefCell::new(Store::new(0)));
        assert_eq!(store.borrow().get_state().state_num, 0);

        let mut reducer = Reducer::new();
        reducer.register_store(Rc::clone(&store));

        reducer.reduce(&Event { new_state_num: 1 });
        assert_eq!(store.borrow().get_state().state_num, 1);

        reducer.reduce(&Event { new_state_num: 0 });
        assert_eq!(store.borrow().get_state().state_num, 0);
    }
}
