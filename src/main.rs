//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//
#[macro_use]
extern crate log;

mod app;
mod configuration;
mod display_control;
mod logging;
mod platform;
mod usb;

fn main() {
    let app = app::App::new();
    app.run();
}
