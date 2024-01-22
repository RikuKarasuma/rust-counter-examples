
fn binary_subset_generator(combo: &Vec<u32>) -> Vec<Vec<&u32>> {
    println!("Finding superset of {:?}", combo);

    // Validation
    if combo.is_empty() {
        return vec![];
    }

    // Generation constants
    const BASE: usize = 2;
    let subset_length: usize = combo.len();
    let generation_length: usize = BASE.pow(subset_length as u32);
    // Initialize possible subset array
    let mut possible_subsets: Vec<u32> = Vec::with_capacity(generation_length);

    let mut possible_combinations: Vec<Vec<&u32>> = Vec::new();

    for i in 0..generation_length {

        let mut b = 1;
        // initialize this index
        possible_subsets.insert(i, 0);

        // Create our unique binary
        let mut number = i as u32;
        while number > 0 {
            let new_index_val = possible_subsets.get(i).unwrap() +
                ((number % 2) * b);

            possible_subsets.insert(i, new_index_val);
            number = number / 2;
            b = b * 10;
        }

        // Reverse that binary into usable combination.
        let mut unique_combo_to_store = Vec::new();
        for x in 0..combo.len() {

            if possible_subsets.get(i).unwrap() % 10 == 1 {
                unique_combo_to_store.push(combo.get(x).unwrap());
            }

            let new_index_val = possible_subsets.get(i).unwrap() / 10;

            possible_subsets.insert(i, new_index_val);
        }

        possible_combinations.push(unique_combo_to_store);

        // println!("i: {}", possible_subsets.get(i).unwrap());
    }

    // If the program made it here, we didn't find anything.
    possible_combinations
}

fn find_knapsack_target(combo: &Vec<u32>, target: u32) -> Vec<&u32> {
    let possible_combinations = binary_subset_generator(combo);

    // Iterate through the possible combos, sum the individual combo,
    // if that meets the target return, the combo.
    for generated_combo in possible_combinations {

        let mut accumulator = 0;

        for x in &generated_combo {
            accumulator += *x;
        }

        if accumulator == target {
            return generated_combo;
        }

        // println!("Combo: {:?}, Summed: {}", generated_combo, accumulator);
    }

    vec![]
}

fn main() {

    let test_vector: Vec<u32> = vec![1, 2, 5, 9, 10];

    println!("Matched: {:?}", find_knapsack_target(&test_vector, 23))

}
