use rust::{instance::Instance, neighbor::Neighborhood, solution::Solution};

fn main() {
    let instance = Instance::from("../instances/abz5.txt");
    let solution = Solution::from(&instance);
    //let t = solution.compute_release_dates(&instance);
    //
    let v = Neighborhood::from(&solution);
    println!("{:?}", solution);
    println!("{v:?}");
    // println!("{instance:?}");
}
