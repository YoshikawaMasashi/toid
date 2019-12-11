use std::boxed::Box;
use std::cell::RefCell;
use std::option::Option;
use std::rc::Rc;

pub struct State {
    state_num: i32,
}

impl State {
    pub fn new(state_num: i32) -> State {
        State { state_num }
    }
    pub fn get_state_num(&self) -> i32 {
        self.state_num
    }

    fn set_state(&mut self, new_state_num: i32) {
        self.state_num = new_state_num;
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    new_state_num: i32,
}

pub struct Observer {
    state: Option<Rc<RefCell<State>>>,
}

impl Observer {
    pub fn new() -> Observer {
        Observer { state: None }
    }
    pub fn set_state(&mut self, state: Rc<RefCell<State>>) {
        self.state = Some(state);
    }

    fn on_notify(&mut self, e: &Event) {
        if let Some(state) = self.state.as_mut() {
            state.borrow_mut().set_state(e.new_state_num);
        } else {
            println!("state is None");
        }
    }
}

trait Subject {
    fn notify_observer(&mut self, e: &Event);
    fn register_observer(&mut self, observer: Box<Observer>);
    fn unregister_observer(&mut self);
}

pub struct ChangeStateNumSubject {
    observer: Option<Box<Observer>>,
}

impl ChangeStateNumSubject {
    pub fn new() -> ChangeStateNumSubject {
        ChangeStateNumSubject { observer: None }
    }
}

impl Subject for ChangeStateNumSubject {
    fn notify_observer(&mut self, e: &Event) {
        if let Some(observer) = self.observer.as_mut() {
            observer.on_notify(e);
        } else {
            println!("observer is None");
        }
    }

    fn register_observer(&mut self, observer: Box<Observer>) {
        self.observer = Some(observer);
    }
    fn unregister_observer(&mut self) {
        self.observer = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_works() {
        let state: Rc<RefCell<State>> = Rc::new(RefCell::new(State::new(0)));
        assert_eq!(state.borrow().get_state_num(), 0);

        let mut observer = Box::new(Observer::new());
        observer.set_state(Rc::clone(&state));

        let mut subject = ChangeStateNumSubject::new();
        subject.register_observer(observer);

        subject.notify_observer(&Event { new_state_num: 1 });
        assert_eq!(state.borrow().get_state_num(), 1);

        subject.notify_observer(&Event { new_state_num: 0 });
        assert_eq!(state.borrow().get_state_num(), 0);
    }
}
