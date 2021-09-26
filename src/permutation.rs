pub(crate) fn generate_permutations_of_n_indices(size: usize) -> Vec<Vec<usize>> {
    let mut initial: Vec<_> = (0..size).into_iter().collect();
    let mut permutations: Vec<Vec<usize>> = vec![];
    generate_permutations(&mut permutations, initial.len(), &mut initial);
    permutations
}

// Heap's recursive permutation algorithm from
// https://en.wikipedia.org/wiki/Heap%27s_algorithm
fn generate_permutations(permutations: &mut Vec<Vec<usize>>, k: usize, a: &mut [usize]) {
    if k == 1 {
        permutations.push(a.to_vec());
    } else {
        // Generate permutations with kth unaltered
        // Initially k == length(A)
        generate_permutations(permutations, k - 1, a);

        // Generate permutations for kth swapped with each k-1 initial
        for i in 0..k - 1 {
            // Swap choice dependent on parity of k (even or odd)
            if k % 2 == 0 {
                a.swap(i, k - 1); // zero-indexed, the kth is at k-1
            } else {
                a.swap(0, k - 1);
            }
            generate_permutations(permutations, k - 1, a);
        }
    }
}
