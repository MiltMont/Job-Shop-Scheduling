use rust::{instance::Instance, solution::Solution};

fn main() {
    let instance = Instance::from("../instances/ft06.txt");
    let mut solution = Solution::from(&instance);
    let t = solution.compute_release_dates(&instance);
    //
    println!("{:?}", t.operations);
    // println!("{instance:?}");
}
