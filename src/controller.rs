use std::sync::RwLock;

use super::model::Calculator;
use super::view::MainWindow;

/// Bridges the view and the model.
pub struct Controller {
    state: RwLock<Calculator>,

    window: slint::Weak<MainWindow>,
}

impl Controller {
    pub fn new(window: slint::Weak<MainWindow>) -> Self {
        Self {
            state: RwLock::new(Calculator::new()),
            window,
        }
    }

    /// Sends a number update to the view.
    fn update_number(&self) {
        let number = self.state.read().as_ref().unwrap().get_number();
        self.window.upgrade().unwrap().set_number(number);
    }

    /// Adds a digit to the end of the displayed number.
    pub fn add_digit(&self, digit: u8) {
        self.state.write().as_mut().unwrap().add_digit(digit);
        self.update_number();
    }

    /// Clears the calculator's state.
    pub fn op_clear(&self) {
        self.state.write().as_mut().unwrap().clear();
        self.update_number();
    }

    /// Attempts to add two numbers.
    pub fn op_add(&self) {
        self.state.write().as_mut().unwrap().add();
        self.update_number();
    }

    /// Attempts to subtract two numbers.
    pub fn op_sub(&self) {
        self.state.write().as_mut().unwrap().sub();
        self.update_number();
    }

    /// Attempts to multiply two numbers.
    pub fn op_mul(&self) {
        self.state.write().as_mut().unwrap().mul();
        self.update_number();
    }

    /// Attempts to divide two numbers.
    pub fn op_div(&self) {
        self.state.write().as_mut().unwrap().div();
        self.update_number();
    }

    /// Attempts to evaluate the current operation.
    pub fn op_eval(&self) {
        self.state.write().as_mut().unwrap().calculate();
        self.update_number();
    }
}
