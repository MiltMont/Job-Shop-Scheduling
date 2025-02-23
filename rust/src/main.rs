use rust::{instance::Instance, solution::Solution};

fn main() {
    let instance = Instance::from("../instances/test01.txt");
    let solution = Solution::from(&instance);
    //let t = solution.compute_release_dates(&instance);
    //
    println!("{:?}", solution);
    // println!("{instance:?}");
}
