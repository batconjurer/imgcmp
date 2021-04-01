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
fn hamming_distance(hash_one: &FixedBitSet, hash_two: &FixedBitSet) -> u8 {
    hash_one.symmetric_difference(hash_two)
        .collect::<FixedBitSet>()
        .count_ones(..) as u8
}

/// The main function that determines if two functions are equal. This works in two steps:
///
/// 1. Compute the `ahash` of both images
/// 2. Check the if the resulting bit-strings are of hamming distance < 10 of each other
pub fn are_images_equal(image_1: &Path, image_2: &Path) -> Result<bool, Error> {
    Ok(hamming_distance(&average_hash(image_1)?,
                        &average_hash(image_2)?) < 10)
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