use rust::{instance::Instance, solution::Solution};

fn main() {
    let instance = Instance::from("../instances/abz7.txt");
    let solution = Solution::from(instance);

    println!("{solution:?}");
}
