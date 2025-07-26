use std::cell::RefCell;

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[derive(Clone, Default)]
struct CounterState {
    value: u64,
    increment: u64,
}

thread_local! {
    static STATE: RefCell<CounterState> = RefCell::new(CounterState::default());
}

#[ic_cdk::init]
fn init(init_value: u64, increment: u64) {
    STATE.with(|state| {
        *state.borrow_mut() = CounterState {
            value: init_value,
            increment,
        };
    });
}

#[ic_cdk::update]
fn increment_counter() {
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.value += s.increment;
    });
}

#[ic_cdk::query]
fn get_counter() -> u64 {
    STATE.with(|state| state.borrow().value)
}
