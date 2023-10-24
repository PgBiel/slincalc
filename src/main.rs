mod controller;
mod model;
mod view;

use std::sync::Arc;

use controller::Controller;
use view::*;

// clone the controller Arc and Window's Weak for each closure (for convenience)
macro_rules! with_cloning {
    (@clone[$var1:ident, $var2:ident] $($b:expr;)*) => {
        $({
            let $var1 = Clone::clone(&$var1);
            let $var2 = Clone::clone(&$var2);
            $b
        };)*
    }
}

fn main() {
    let window: MainWindow = MainWindow::new().unwrap();
    let window_weak = window.as_weak();
    let controller = Arc::new(Controller::new());

    with_cloning! {
        @clone[controller, window_weak]

        window.on_add_digit(move |digit| {
            controller.add_digit(window_weak.upgrade().unwrap(), digit.try_into().unwrap());
        });

        window.on_op_add(move || {
            controller.op_add(window_weak.upgrade().unwrap());
        });

        window.on_op_sub(move || {
            controller.op_sub(window_weak.upgrade().unwrap());
        });

        window.on_op_mul(move || {
            controller.op_mul(window_weak.upgrade().unwrap());
        });

        window.on_op_div(move || {
            controller.op_div(window_weak.upgrade().unwrap());
        });

        window.on_op_eval(move || {
            controller.op_eval(window_weak.upgrade().unwrap());
        });

        window.on_op_clear(move || {
            controller.op_clear(window_weak.upgrade().unwrap());
        });
    }

    window.run().unwrap();
}
