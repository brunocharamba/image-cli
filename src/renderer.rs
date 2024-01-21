use image::imageops::FilterType;
use crate::enums::Direction;

pub fn blur(source_image: &str, target_image: &str, value: f32) {
  println!("Blurring image {} to {} using {}", &source_image, &target_image, &value);
  let original = image::open(source_image).expect("Failed when opening the image.");

  let changed = original.blur(value);
  changed.save(target_image).expect("Failed to save the image.");
}

pub fn rotate(source_image: &str, target_image: &str, direction: Direction, value: u32) {
  println!("Rotating image {} to {}.\nRotate {} degrees to the {}", &source_image, &target_image, &value, &direction.to_string());
  let original = image::open(source_image).expect("Failed when opening the image.");

  let changed = match value {
    90 => if direction == Direction::Right { original.rotate90() } else { original.rotate270() }
    270 => if direction == Direction::Right { original.rotate270() } else { original.rotate90() }
    _ => original.rotate180(),
  };

  changed.save(target_image).expect("Failed to save the image.");
}

pub fn resize(source_image: &str, target_image: &str) {
  let original = image::open(source_image).expect("Failed when opening the image.");
  original.resize(100, 100, FilterType::CatmullRom);
}