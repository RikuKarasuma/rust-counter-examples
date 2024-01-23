
mod knapsack_tests;
mod knapsack;

fn main() {
    // Refer to knapsack_tests.rs, for more in depth scenarios'.
    let test_vector: Vec<u32> = vec![1, 2, 5, 9, 10];
    println!("Matched: {:?}", knapsack::find_knapsack_target(&test_vector, 22))
}
