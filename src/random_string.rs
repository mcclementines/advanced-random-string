//! src/generate.rs

use rand::{
    distributions::Uniform,
    rngs::{OsRng, SmallRng},
    thread_rng, Rng, SeedableRng,
};

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
/// use advanced_random_string::{charset, random_string};
///
/// let random_string = random_string::generate(10, charset::BASE62);
/// println!("Generated string: {}", random_string);
///
/// // Specify a custom charset
/// let charset = b"MY_CHARSET";
/// let random_string_with_custom_charset = random_string::generate(10, charset);
/// println!("Generated string: {}", random_string_with_custom_charset);
/// ```
pub fn generate(length: usize, charset: &[u8]) -> String {
    let mut rng = thread_rng();

    generate_with_rng(length, charset, &mut rng)
}

/// Generates a unsecure random string with SmallRng of a specified length using a given character set.
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
/// use advanced_random_string::{charset, random_string};
///
/// let random_string = random_string::generate_unsecure(10, charset::BASE62);
/// println!("Generated string: {}", random_string);
/// ```
pub fn generate_unsecure(length: usize, charset: &[u8]) -> String {
    let mut rng = SmallRng::from_entropy();

    generate_with_rng(length, charset, &mut rng)
}

/// Generates a secure random string with OsRng of a specified length using a given character set.
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
/// use advanced_random_string::{charset, random_string};
///
/// let random_string = random_string::generate_os_secure(10, charset::BASE62);
/// println!("Generated string: {}", random_string);
/// ```
pub fn generate_os_secure(length: usize, charset: &[u8]) -> String {
    let mut rng = OsRng;

    generate_with_rng(length, charset, &mut rng)
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
/// use advanced_random_string::{charset, random_string};
///
/// let mut rng = SmallRng::from_entropy();
/// let random_string = random_string::generate_with_rng(10, charset::BASE62, &mut rng);
/// println!("Generated string: {}", random_string);
/// ```
pub fn generate_with_rng<R: Rng>(length: usize, charset: &[u8], rng: &mut R) -> String {
    let uniform = Uniform::from(0..charset.len());

    (0..length)
        .map(|_| {
            let idx = rng.sample(uniform);
            charset[idx] as char
        })
        .collect()
}
