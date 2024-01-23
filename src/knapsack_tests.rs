/*
 * Find counterexamples to each of the following algorithms for the knapsack prob-
 * lem. That is, give an S and T where the algorithm does not ﬁnd a solution that
 * leaves the knapsack completely full, even though a full-knapsack solution exists
 */
#[cfg(test)]
mod tests {
    use crate::knapsack;

    #[test]
    fn should_meet_target_using_binary_subset_algo() {
        // Test data + expected pointers
        let test_vector: Vec<u32> = vec![1, 2, 5, 9, 10];
        let expected_subset: Vec<&u32> = vec![&1, &2, &9, &10];
        // Find subset pointers which match our target value.
        let target_subset = knapsack::find_knapsack_target(&test_vector, 22);
        // Compare the pointers of both vectors.
        assert_eq!(expected_subset, target_subset);
    }

    #[test]
    fn should_meet_target_using_first_fit_algo() {
        // Test data + expected pointers
        let test_vector: Vec<u32> = vec![10, 9, 2, 1, 5];
        let expected_subset: Vec<&u32> = vec![&10, &9, &2, &1];
        // Find subset pointers which match our target value.
        let target_subset = knapsack::find_knapsack_target_first_fit(&test_vector, 22);
        // Compare the pointers of both vectors.
        assert_eq!(expected_subset, target_subset);
    }

    /*
     * (a) Put the elements of S in the knapsack in left to right order if they ﬁt, that
     *     is, the ﬁrst-ﬁt algorithm.
     */
    #[test]
    fn should_fail_to_meet_target_using_first_fit_algo_counter_example() {
        // Test data + expected pointers
        let test_vector: Vec<u32> = vec![1, 2, 5, 9, 10];
        let expected_subset: Vec<&u32> = vec![];
        // Find subset pointers which match our target value.
        let target_subset = knapsack::find_knapsack_target_first_fit(&test_vector, 22);
        // Compare the pointers of both vectors.
        assert_eq!(expected_subset, target_subset);
    }
}