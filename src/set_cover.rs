/*
 *  1-6. [5] The set cover problem is as follows: given a set S of subsets
 *  {S1, . . . , Sm} of the universal set U = {1, ..., n}, find the smallest
 *  subset of subsets T ⊆ S such that ∪ti ∈T ti = U . For example, consider
 *  the subsets S1 = {1, 3, 5}, S2 = {2, 4}, S3 = {1, 4}, and S4 = {2, 5}.
 *  The set cover of {1, . . . , 5} would then be S1 and S2.
 *
 *  Find a counterexample for the following algorithm: Select the largest
 *  subset for the cover, and then delete all its elements from the universal
 *  set. Repeat by adding the subset containing the largest number of uncovered
 *  elements until all are covered.
 */
pub fn find_universal_set(subsets: &mut Vec<Vec<u32>>,
                          universal_set: &mut Vec<u32>,
                          by_largest: bool) -> Vec<Vec<u32>> {

    // Sort our subsets in ascending order so that we find the smallest first.
    subsets.sort_by_key(|a| a.len());
    // Reverse the order after sort if we want descending size.
    if by_largest {
        subsets.reverse();
    }

    println!("Subsets: {:?}", subsets);
    // Use this copy to track the covered elements.
    let mut trackable_universal_set = universal_set.clone();
    let mut covered_subsets: Vec<Vec<u32>> = Vec::new();

    // Iterate through each subset...
    for subset in subsets {
        // Check if any element in the subset is what we are looking for in
        // our universal set.
        if trackable_universal_set.iter().any(|a| subset.contains(a)) {
            // Remove that which we have found.
            trackable_universal_set.retain(|a| !subset.contains(a));
            // Add our found subset to the covered subsets vector.
            covered_subsets.push(subset.clone());
            println!("Contained all! remaining: {:?}", trackable_universal_set);
        }
    }

    // If we still have elements, report back that we couldn't cover all universal
    // elements with an empty vector.
    if !trackable_universal_set.is_empty() {
        return vec![];
    }

    covered_subsets
}