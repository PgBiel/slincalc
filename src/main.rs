mod controller;
mod model;
mod view;

use std::sync::Arc;

use controller::Controller;
use view::*;

fn main() {
    let window: MainWindow = MainWindow::new().unwrap();
    let controller = Arc::new(Controller::new());

    controller.register_callback(&window, |controller, window_weak| {
        window.on_add_digit(move |digit| {
            controller.add_digit(window_weak.upgrade().unwrap(), digit.try_into().unwrap());
        })
    });

    controller.register_callback(&window, |controller, window_weak| {
        window.on_op_add(move || {
            controller.op_add(window_weak.upgrade().unwrap());
        })
    });

    controller.register_callback(&window, |controller, window_weak| {
        window.on_op_sub(move || {
            controller.op_sub(window_weak.upgrade().unwrap());
        })
    });

    controller.register_callback(&window, |controller, window_weak| {
        window.on_op_mul(move || {
            controller.op_mul(window_weak.upgrade().unwrap());
        })
    });

    controller.register_callback(&window, |controller, window_weak| {
        window.on_op_div(move || {
            controller.op_div(window_weak.upgrade().unwrap());
        })
    });

    controller.register_callback(&window, |controller, window_weak| {
        window.on_op_eval(move || {
            controller.op_eval(window_weak.upgrade().unwrap());
        })
    });

    controller.register_callback(&window, |controller, window_weak| {
        window.on_op_clear(move || {
            controller.op_clear(window_weak.upgrade().unwrap());
        })
    });

    window.run().unwrap();
}
