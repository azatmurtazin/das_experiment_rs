pub mod int_num_examples;
pub mod kv_examples;
pub mod list_examples;
pub mod map_examples;
pub mod text_examples;

pub fn run() {
    println!("### Dynamic types example");

    int_num_examples::run();
    text_examples::run();
    kv_examples::run();
    list_examples::run();
    map_examples::run();
}
