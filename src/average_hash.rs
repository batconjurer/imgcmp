use std::path::Path;
use image::io::Reader as ImageReader;
use image::imageops::FilterType;
use fixedbitset::FixedBitSet;

use crate::errors::Error;


/// Computes a perceptual hash of an image as described at
/// https://www.hackerfactor.com/blog/index.php?/archives/432-Looks-Like-It.html
///
/// The algorithm produces 64 bit hash of an image according to the following procedure:
/// 1. The image is resize to and 8 x 8 array of 64 pixels.
/// 2. The image is then converted to grayscale
/// 3. The average color value is computed
/// 4. Each bit in the hash indicates if the color in the corresponding pixel exceeds the average
fn average_hash(image_path: &Path) -> Result<FixedBitSet, Error> {
    // Decode the image, resize it into an 8 x 8 grid an convert to grayscale
    let img = ImageReader::open(image_path)?
        .decode()?
        .resize_exact(8, 8, FilterType::Nearest)
        .grayscale();
    // Compute the average color value of the resulting image
    let average: u32 = img.to_bytes().iter().map(|b| *b as u32).sum::<u32>() / 64;
    // Initialize a set of 64 bits (all initially 0)
    let mut hash = FixedBitSet::with_capacity(64);
    // For each pixel exceeding the average color value, set its corresponding bit to 1
    let _ = img.to_bytes()
        .iter()
        .enumerate()
        .map(|(ix, pixel)| hash.set(ix, *pixel as u32 <= average))
        .collect::<()>();
    Ok(hash)
}

/// Computes the standard hamming distance of two bit-strings. This is the number of places where
/// the two strings differ.
fn hamming_distance(hash_one: &FixedBitSet, hash_two: &FixedBitSet) -> Result<u8, Error> {
    if hash_one.len() != hash_two.len() {
        Err(Error::LengthMismatch)
    } else {
        Ok(hash_one.symmetric_difference(hash_two)
            .collect::<FixedBitSet>()
            .count_ones(..) as u8)
    }
}

/// The main function that determines if two functions are equal. This works in two steps:
///
/// 1. Compute the `ahash` of both images
/// 2. Check the if the resulting bit-strings are of hamming distance < 10 of each other
pub fn are_images_equal(image_1: &Path, image_2: &Path) -> Result<bool, Error> {
    Ok(hamming_distance(&average_hash(image_1)?,
                        &average_hash(image_2)?)? < 10)
}

/// Test that the hamming distance function works correctly
#[cfg(test)]
mod test_hamming {
    use super::*;

    /// A template for the parameterized tests below
    fn hamming_distance_test_template(distance: u8, hash_one: Vec<bool>, hash_two: Vec<bool>) {
        let mut hash_one_ = FixedBitSet::with_capacity(hash_one.len());
        let mut hash_two_ = FixedBitSet::with_capacity(hash_two.len());

        let _ = hash_one.iter()
            .enumerate()
            .map(|(ix, value)| hash_one_.set(ix, *value))
            .collect::<()>();

        let _ = hash_two.iter()
            .enumerate()
            .map(|(ix, value)| hash_two_.set(ix, *value))
            .collect::<()>();

        let expected_distance = hamming_distance(&hash_one_, &hash_two_).unwrap();
        assert_eq!(expected_distance, distance);
    }

    macro_rules! hamming_distance_tests {
        ($($name: ident: ($distance:expr, $hash_one:expr, $hash_two:expr, $panic:meta),)*) => {
            $(
                #[test]
                #[$panic]
                fn $name() {
                    hamming_distance_test_template($distance, $hash_one, $hash_two);
                }
            )*
        };
    }

    hamming_distance_tests!{
        test_hamming_distance_panic: (
            0,
            vec!(true),
            vec!(true, false),
            should_panic
        ),
        test_hamming_distance_panic_2: (
            0,
            Vec::new(),
            vec!(true, false),
            should_panic
        ),
        test_hamming_distance_trivial: (
            0,
            Vec::new(),
            Vec::new(),
            test
        ),
        test_hamming_distance_0: (
            0,
            vec!(true, false, true, false),
            vec!(true, false, true, false),
            test
        ),
        test_hamming_distance_1: (
            1,
            vec!(false, false, true, false),
            vec!(true, false, true, false),
            test
        ),
        test_hamming_distance_4: (
            4,
            vec!(false, false, true, true),
            vec!(true, true, false, false),
            test
        ),

    }
}


/// This is minimum required tests that must pass in order for the assignment to be complete.
#[cfg(test)]
mod test_assets {
    use super::*;

    #[test]
    fn test_asset_comparisons()  {
        assert!(are_images_equal(Path::new("assets/cat.jpg"),
                                 Path::new("assets/cat_edited.jpg"))
            .expect("Test failed"));
        assert!(are_images_equal(Path::new("assets/cat2.jpg"),
                                 Path::new("assets/cat2_edited.jpg"))
            .expect("Test failed"));
        assert!(are_images_equal(Path::new("assets/ferrari_roma.jpg"),
                                 Path::new("assets/ferrari_roma_edited.png"))
            .expect("Test failed"));
        assert!(are_images_equal(Path::new("assets/ferrari_roma2.jpg"),
                                 Path::new("assets/ferrari_roma2_edited.jpg"))
            .expect("Test failed"));
        assert!(!are_images_equal(Path::new("assets/cat2.jpg"),
                                 Path::new("assets/cat_edited.jpg"))
            .expect("Test failed"));
        assert!(!are_images_equal(Path::new("assets/ferrari_roma.jpg"),
                                  Path::new("assets/cat_edited.jpg"))
            .expect("Test failed"));
        assert!(!are_images_equal(Path::new("assets/cat.jpg"),
                                  Path::new("assets/cat2_edited.jpg"))
            .expect("Test failed"));
        assert!(!are_images_equal(Path::new("assets/ferrari_roma2.jpg"),
                                  Path::new("assets/ferrari_roma_edited.png"))
            .expect("Test failed"));
        assert!(!are_images_equal(Path::new("assets/ferrari_roma.jpg"),
                                  Path::new("assets/ferrari_roma2_edited.jpg"))
            .expect("Test failed"));
    }
}