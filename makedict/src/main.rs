use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let full_dict_path = &args[1];
    let computed_dict_path = &args[2];

    println!("full_dict_path: {}", full_dict_path);
    println!("computed_dict_path: {}", computed_dict_path);
}
