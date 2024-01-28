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
#[cfg(test)]
mod tests {
    use crate::set_cover;

    const EXPECTED_EMPTY_TARGET: Vec<Vec<u32>> = vec![];

    #[test]
    fn should_meet_target_universal_set_smallest_first() {
        // Test data.
        let mut test_subsets: Vec<Vec<u32>> = vec![
            vec![1, 2, 5],
            vec![10, 100],
            vec![1, 2, 5, 9, 101],
            vec![101, 102, 103, 104],
            vec![105, 106, 107, 108],
        ];
        let mut universal_set: Vec<u32> = vec![1, 2, 9, 10, 100];
        let expected_subsets: Vec<Vec<u32>> = vec![
            vec![10, 100],
            vec![1, 2, 5],
            vec![1, 2, 5, 9, 101],
        ];

        // Attempt to find subsets which cover the universal set.
        let set_covered_subsets = set_cover::find_universal_set(&mut test_subsets, &mut universal_set, false);
        // Compare the data of both vectors.
        assert_eq!(expected_subsets, set_covered_subsets);
    }

    #[test]
    fn should_not_meet_target_universal_set_smallest_first() {
        // Test data.
        let mut test_subsets: Vec<Vec<u32>> = vec![
            vec![1, 2,],
            vec![1, 2, 5],
            vec![10, 100],
            vec![1, 2, 5, 9, 101],
            vec![101, 102, 103, 104],
            vec![105, 106, 107, 108],
        ];
        let mut universal_set: Vec<u32> = vec![1, 2, 9, 10, 1000];

        // Attempt to find subsets which cover the universal set.
        let set_covered_subsets = set_cover::find_universal_set(&mut test_subsets, &mut universal_set, false);
        // // Compare the data of both vectors.
        assert_eq!(EXPECTED_EMPTY_TARGET, set_covered_subsets);
    }

    #[test]
    fn should_meet_target_universal_set_biggest_first() {
        // Test data.
        let mut test_subsets: Vec<Vec<u32>> = vec![
            vec![1, 2, 5],
            vec![10, 100],
            vec![1, 2, 5, 9, 101],
            vec![101, 102, 103, 104],
            vec![105, 106, 107, 108],
        ];
        let mut universal_set: Vec<u32> = vec![1, 2, 9, 10, 100];
        let expected_subsets: Vec<Vec<u32>> = vec![
            vec![1, 2, 5, 9, 101],
            vec![10, 100],
        ];

        // Attempt to find subsets which cover the universal set.
        let set_covered_subsets = set_cover::find_universal_set(&mut test_subsets, &mut universal_set, true);
        // Compare the data of both vectors.
        assert_eq!(expected_subsets, set_covered_subsets);
    }

    #[test]
    fn should_not_meet_target_universal_set_biggest_first() {
        // Test data.
        let mut test_subsets: Vec<Vec<u32>> = vec![
            vec![1, 2, 5],
            vec![10, 100],
            vec![1, 2, 5, 9, 101],
            vec![101, 102, 103, 104],
            vec![105, 106, 107, 108, 109, 110],
        ];
        let mut universal_set: Vec<u32> = vec![1, 2, 9, 10, 100, 1000];

        // Attempt to find subsets which cover the universal set.
        let set_covered_subsets = set_cover::find_universal_set(&mut test_subsets, &mut universal_set, true);
        // Compare the data of both vectors.
        assert_eq!(EXPECTED_EMPTY_TARGET, set_covered_subsets);
    }

}