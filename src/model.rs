/// The calculator's state.
pub struct Calculator {
    /// 'number: None' means we just made an operation with the previous number.
    /// The previous number would then be held in 'op'.
    number: Option<i32>,
    op: Option<Operation>,
}

impl Default for Calculator {
    fn default() -> Self {
        Self {
            number: Some(0),
            op: None,
        }
    }
}

impl Calculator {
    /// Creates a new calculator with the value 0.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number to display.
    /// Usually corresponds to the `number` field.
    /// However, after using an operation, this will correspond
    /// to the _previous_ number, stored in the operation object itself.
    pub fn get_number(&self) -> i32 {
        self.number
            .unwrap_or_else(|| self.op.unwrap().held_number())
    }

    /// Adds a digit to the currently displayed number.
    pub fn add_digit(&mut self, digit: u8) {
        assert!(digit < 10);
        self.number = Some(
            self.number
                .unwrap_or(0)
                .saturating_mul(10)
                .saturating_add(digit as i32),
        );
    }

    /// Clears the calculator's current state, setting its number to zero
    /// and removing pending operations.
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// If possible, applies the select operation between the two numbers.
    pub fn calculate(&mut self) {
        if let Some(op) = self.op.take() {
            self.number = Some(op.calculate(self.number.unwrap_or(0)));
        }
    }

    /// Executes the 'add' operation.
    pub fn add(&mut self) {
        self.calculate();
        let lhs = self.number.take().unwrap_or(0);
        self.op = Some(Operation::Add(lhs));
    }

    /// Executes the 'sub' operation.
    pub fn sub(&mut self) {
        self.calculate();
        let lhs = self.number.take().unwrap_or(0);
        self.op = Some(Operation::Sub(lhs));
    }

    /// Executes the 'mul' operation.
    pub fn mul(&mut self) {
        self.calculate();
        let lhs = self.number.take().unwrap_or(0);
        self.op = Some(Operation::Mul(lhs));
    }

    /// Executes the 'div' operation.
    pub fn div(&mut self) {
        self.calculate();
        let lhs = self.number.take().unwrap_or(0);
        self.op = Some(Operation::Div(lhs));
    }
}

/// A binary operation.
#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add(i32),
    Sub(i32),
    Mul(i32),
    Div(i32),
}

impl Operation {
    /// Apply this operation to two numbers, returning a result.
    pub fn calculate(self, number2: i32) -> i32 {
        match self {
            Operation::Add(number1) => number1.saturating_add(number2),
            Operation::Sub(number1) => number1.saturating_sub(number2),
            Operation::Mul(number1) => number1.saturating_mul(number2),
            Operation::Div(_) if number2 == 0 => 0, // quick workaround for div by 0
            Operation::Div(number1) => number1.saturating_div(number2),
        }
    }

    /// Returns the first number held by this operation.
    pub fn held_number(self) -> i32 {
        match self {
            Operation::Add(number) => number,
            Operation::Sub(number) => number,
            Operation::Mul(number) => number,
            Operation::Div(number) => number,
        }
    }
}
