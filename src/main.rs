mod random;

fn main() {
    println!("Hello, world!");

    let v = vec![1.0, 2.0, 3.0, 4.0, 5.1];

    println!("Sum of vec: {}", sum_vec_imperative(&v));
    println!("Sum of vec: {}", sum_vec_iterative(&v));

    // println!("v before assignment: {:?}", v);

    // let mut v2 = &v;
    // println!("v2 after assignment: {:?}", v2);
    // v[0] = 100.0;
    // v2[0] = 100.0;
    // println!("v after assignment: {:?}", v);

    let points = random::generate_random_points(100_000, 1.0);
    let other_points = random::generate_random_points(100_000, 1.0);

    let distances = distances::nearest_distances(points, other_points);

    let result = sum_vec_iterative(&distances);
    dbg!(&result);
}

/// This function sums all the elements of a vector in an imperative way
fn sum_vec_imperative(v: &[f64]) -> f64 {
    let mut sum = 0.0;
    for i in 0..v.len() {
        sum += v[i];
    }
    sum
}

/// This function is equivalent to the above function
fn sum_vec_iterative(v: &[f64]) -> f64 {
    v.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_vec_imperative() {
        let v = vec![1.0, 2.0, 3.0, 4.0, 5.1];
        assert!((sum_vec_imperative(&v) - 15.1).abs() < 1e-6);
    }

    #[test]
    fn test_sum_vec_iterative() {
        let v = vec![1.0, 2.0, 3.0, 4.2, 5.1];
        assert!((sum_vec_iterative(&v) - 15.1).abs() < 1e-6);
    }
}
