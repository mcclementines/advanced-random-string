//! src/generate.rs

use rand::{distributions::Uniform, thread_rng, Rng};

/// Generates a random string of a specified length using a given character set.
///
/// # Arguments
///
/// * `length` - The length of the generated string.
/// * `charset` - A slice of bytes representing the character set to use for generating the string.
///
/// # Returns
///
/// A `String` containing randomly selected characters from the provided character set.
///
/// # Examples
///
/// ```
/// use advanced_random_string::{charset, generate};
///
/// let random_string = generate(10, charset::BASE62);
/// println!("Generated string: {}", random_string);
/// 
/// // Specify a custom charset
/// let charset = b"MY_CHARSET";
/// let random_string_with_custom_charset = generate(10, charset);
/// println!("Generated string: {}", random_string_with_custom_charset);
/// ```
pub fn generate(length: usize, charset: &[u8]) -> String {
    let mut rng = thread_rng();
    let uniform = Uniform::from(0..charset.len());

    (0..length)
        .map(|_| {
            let idx = rng.sample(&uniform);
            charset[idx] as char
        })
        .collect::<String>()
}

/// Generates a random string of a specified length using a given character set and RNG.
///
/// # Arguments
///
/// * `length` - The length of the generated string.
/// * `charset` - A slice of bytes representing the character set to use for generating the string.
/// * `rng` - A mutable reference to an RNG implementing the `Rng` trait.
///
/// # Returns
///
/// A `String` containing randomly selected characters from the provided character set.
///
/// # Examples
///
/// ```
/// use rand::SeedableRng;
/// use rand::rngs::SmallRng;
/// use advanced_random_string::{charset, generate_with_rng};
///
/// let mut rng = SmallRng::from_entropy();
/// let random_string = generate_with_rng(10, charset::BASE62, &mut rng);
/// println!("Generated string: {}", random_string);
/// ```
pub fn generate_with_rng<R: Rng>(length: usize, charset: &[u8], rng: &mut R) -> String {
    let uniform = Uniform::from(0..charset.len());

    (0..length)
        .map(|_| {
            let idx = rng.sample(&uniform);
            charset[idx] as char
        })
        .collect()
}
