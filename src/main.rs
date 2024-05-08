use controller::TodolistRS;

mod constants;
mod controller;
mod models;
mod utils;

fn main() {
    TodolistRS::new().run();
}
