
fn subset_matcher(subset: &Vec<u16>, target: u16) -> Vec<&u16> {
    println!("Finding target {} within subset {:?}", target, subset);

    // Validation
    if subset.is_empty() {
        return vec![];
    }


    let mut target_subset: Vec<&u16> = Vec::new();

    // This won't work because it doesn't consider non-contiguous subsets
    // Need to consider a subset generation function.
    //
    // if not, use a two pointer - expanding - sliding window to find our
    // target subset.
    //
    // let mut sliding_window: u16 = 1;
    // for x_window in 0..subset.len() {
    //     for y_window in subset.windows(sliding_window as usize + x_window) {
    //         println!("window {:?}", y_window);
    //
    //         // Sum our window and check against target if we found the
    //         // target return a new vector referenced to our original data.
    //         if y_window.iter().sum::<u16>() == target {
    //             target_subset.extend(y_window);
    //
    //             return target_subset;
    //         }
    //     }
    // }

    // If the program made it here, we didn't find anything.
    vec![]
}

fn main() {

    let test_vector: Vec<u16> = vec![1, 2, 5, 9, 10];



    println!("Matched: {:?}", subset_matcher(&test_vector, 22))

}
