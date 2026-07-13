#[macro_use]
mod macros;

mod granular_mod_example;
mod types;
mod types_examples;
mod user;

fn main() {
    granular_mod_example::run();
    types_examples::run();
}
