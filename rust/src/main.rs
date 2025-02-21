use rust::{instance::Instance, solution::Solution};

fn main() {
    let instance = Instance::from("../instances/ft06.txt");
    let solution = Solution::from(&instance);

    // println!("{instance:?}");
    println!("{solution:?}");
}
