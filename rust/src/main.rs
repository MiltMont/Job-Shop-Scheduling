use rust::instance::Instance;

fn main() {
    let instance = Instance::from("../instances/abz7.txt");

    println!("{instance:?}");
}
