use std::path::Path;
use image::io::Reader as ImageReader;
use image::imageops::FilterType;


use crate::errors::Error;

pub fn ahash(image_path: &Path) -> Result<[bool; 64], Error> {
    let img = ImageReader::open(image_path)?
        .decode()?
        .resize_exact(8, 8, FilterType::Nearest)
        .grayscale();
    let average: u32 = img.to_bytes().iter().map(|b| *b as u32).sum::<u32>() / 64;
    let mut hash: [bool; 64] = [false; 64];
    let _ = img.to_bytes()
        .iter()
        .enumerate()
        .map(|(ix, pixel)| hash[ix] = *pixel as u32 <= average)
        .collect::<()>();
    Ok(hash)
}

pub fn hamming_distance(hash_one: &[bool; 64], hash_two: &[bool; 64]) -> u8 {
    hash_one.iter()
        .zip(hash_two.iter())
        .map(|(first, second)| if first==second { 0 } else { 1 })
        .sum::<u8>()
}

pub fn are_images_equal(image_1: &Path, image_2: &Path) -> Result<bool, Error> {
    Ok(hamming_distance(&ahash(image_1)?,
                        &ahash(image_2)?) < 10)
}

#[cfg(test)]
mod test_ahash {
    use super::*;

    #[test]
    fn test_hash_equal_1()  {
        assert!(are_images_equal(Path::new("assets/cat.jpg"),
                                 Path::new("assets/cat_edited.jpg"))
            .expect("Test failed"));
    }

    #[test]
    fn test_hash_equal_2() {
        assert!(are_images_equal(Path::new("assets/cat2.jpg"),
                                 Path::new("assets/cat2_edited.jpg"))
            .expect("Test failed"));
    }

    #[test]
    fn test_hash_equal_3() {
        assert!(are_images_equal(Path::new("assets/ferrari_roma.jpg"),
                                 Path::new("assets/ferrari_roma_edited.png"))
            .expect("Test failed"));
    }

    #[test]
    fn test_has_equal_4() {
        assert!(are_images_equal(Path::new("assets/ferrari_roma2.jpg"),
                                 Path::new("assets/ferrari_roma2_edited.jpg"))
            .expect("Test failed"));
    }

    #[test]
    fn test_not_equal_1() {
        assert!(!are_images_equal(Path::new("assets/cat2.jpg"),
                                 Path::new("assets/cat_edited.jpg"))
            .expect("Test failed"));
    }

    #[test]
    fn test_not_equal_2() {
        assert!(!are_images_equal(Path::new("assets/ferrari_roma.jpg"),
                                  Path::new("assets/cat_edited.jpg"))
            .expect("Test failed"));
    }

    #[test]
    fn test_not_equal_3() {
        assert!(!are_images_equal(Path::new("assets/cat.jpg"),
                                  Path::new("assets/cat2_edited.jpg"))
            .expect("Test failed"));
    }

    #[test]
    fn test_not_equal_4() {
        assert!(!are_images_equal(Path::new("assets/ferrari_roma2.jpg"),
                                  Path::new("assets/ferrari_roma_edited.png"))
            .expect("Test failed"));
    }

    #[test]
    fn test_not_equal_5() {
        assert!(!are_images_equal(Path::new("assets/ferrari_roma.jpg"),
                                  Path::new("assets/ferrari_roma2_edited.jpg"))
            .expect("Test failed"));
    }
}