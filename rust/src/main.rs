use rust::{instance::Instance, solution::Solution};

fn main() {
    let instance = Instance::from("../instances/dmu55.txt");
    let solution = Solution::from(&instance);

    println!("{instance:?}");
    println!("{solution:?}");
}
