use rand::Rng;

fn generate_random_points(num_points: usize, extent: f64) -> Vec<(f64, f64)> {
    let mut rng = rand::thread_rng();

    (0..num_points)
        .map(|_| {
            (
                rng.gen_range(-extent..extent),
                rng.gen_range(-extent..extent),
            )
        })
        .collect()
}

fn main() {
    let points = generate_random_points(5000, 1.0);

    let distances = distances::nearest_distances(points);
    dbg!(distances);
}
