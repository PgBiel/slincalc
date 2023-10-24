mod model;
mod view;
mod controller;

use std::sync::Arc;

use controller::Controller;
use view::*;

// clone the controller Arc for each closure (for convenience)
macro_rules! with_controller {
    ([$controller:ident] $($b:expr;)*) => {
        $({
            let $controller = Arc::clone(&$controller);
            $b
        };)*
    }
}

fn main() {
    let window: MainWindow = MainWindow::new().unwrap();
    let controller = Arc::new(Controller::new(window.as_weak()));

    with_controller! {
        [controller]

        window.on_add_digit(move |digit| {
            controller.add_digit(digit.try_into().unwrap());
        });

        window.on_op_add(move || {
            controller.op_add();
        });

        window.on_op_sub(move || {
            controller.op_sub();
        });

        window.on_op_mul(move || {
            controller.op_mul();
        });

        window.on_op_div(move || {
            controller.op_div();
        });

        window.on_op_eval(move || {
            controller.op_eval();
        });

        window.on_op_clear(move || {
            controller.op_clear();
        });
    }

    window.run().unwrap();
}
