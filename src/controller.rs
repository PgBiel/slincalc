use std::sync::{Arc, RwLock};

use super::model::Calculator;
use super::view::MainWindow;

/// Bridges the view and the model.
pub struct Controller {
    state: RwLock<Calculator>,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            state: RwLock::new(Calculator::new()),
        }
    }

    /// Registers a callback with cloned controller and window references.
    pub fn register_callback(
        self: &Arc<Self>,
        window: &MainWindow,
        callback: impl FnOnce(Arc<Self>, slint::Weak<MainWindow>),
    ) {
        let controller = Arc::clone(self);
        let window = <MainWindow as slint::ComponentHandle>::as_weak(window);
        callback(controller, window);
    }

    /// Sends a number update to the view.
    fn update_number(&self, window: MainWindow) {
        let number = self.state.read().as_ref().unwrap().get_number();
        window.set_number(number);
    }

    /// Adds a digit to the end of the displayed number.
    pub fn add_digit(&self, window: MainWindow, digit: u8) {
        self.state.write().as_mut().unwrap().add_digit(digit);
        self.update_number(window);
    }

    /// Clears the calculator's state.
    pub fn op_clear(&self, window: MainWindow) {
        self.state.write().as_mut().unwrap().clear();
        self.update_number(window);
    }

    /// Attempts to add two numbers.
    pub fn op_add(&self, window: MainWindow) {
        self.state.write().as_mut().unwrap().add();
        self.update_number(window);
    }

    /// Attempts to subtract two numbers.
    pub fn op_sub(&self, window: MainWindow) {
        self.state.write().as_mut().unwrap().sub();
        self.update_number(window);
    }

    /// Attempts to multiply two numbers.
    pub fn op_mul(&self, window: MainWindow) {
        self.state.write().as_mut().unwrap().mul();
        self.update_number(window);
    }

    /// Attempts to divide two numbers.
    pub fn op_div(&self, window: MainWindow) {
        self.state.write().as_mut().unwrap().div();
        self.update_number(window);
    }

    /// Attempts to evaluate the current operation.
    pub fn op_eval(&self, window: MainWindow) {
        self.state.write().as_mut().unwrap().calculate();
        self.update_number(window);
    }
}
