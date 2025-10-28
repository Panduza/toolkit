use rand::{distributions::Alphanumeric, Rng};

// -------------------------------------------------------------------------------

/// Generate a random string of the specified length using alphanumeric characters
pub fn generate_random_string(length: usize) -> String {
    let rng = rand::thread_rng();
    rng.sample_iter(Alphanumeric)
        .take(length)
        .map(|c| c as char)
        .collect()
}

// -------------------------------------------------------------------------------
