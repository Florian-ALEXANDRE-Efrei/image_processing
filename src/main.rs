use image_processing::filters::{edge_detection, morphological};
use image_processing::utils::image_utils;

fn main() {
    let img = image_utils::get_image("images_src/Bureau.png");
    let img_prewitt = edge_detection::prewitt(img.clone());
    img_prewitt.save("images_res/bureau_prewitt.png").expect("Failed to save image");

    let img_sobel = edge_detection::sobel(img.clone());
    img_sobel.save("images_res/bureau_sobel.png").expect("Failed to save image");

    let img_kirsch = edge_detection::kirsch(img.clone());
    img_kirsch.save("images_res/bureau_kirsch.png").expect("Failed to save image");

    let img_robinson = edge_detection::robinson(img.clone());
    img_robinson.save("images_res/bureau_robinson.png").expect("Failed to save image");

    let img_threshold = image_utils::thresholding(img_prewitt, 30);
    img_threshold.save("images_res/bureau_thresholding.png").expect("Failed to save image");

    let gabarit = [
        [image_utils::WHITEPIXEL, image_utils::BLACKPIXEL, image_utils::WHITEPIXEL],
        [image_utils::BLACKPIXEL, image_utils::BLACKPIXEL, image_utils::BLACKPIXEL],
        [image_utils::WHITEPIXEL, image_utils::BLACKPIXEL, image_utils::WHITEPIXEL],
    ];

    let img_dilatee = morphological::dilatation(&gabarit, img_threshold.clone());
    img_dilatee.save("images_res/bureau_dilatee.png").expect("Failed to save image");

    let img_erodee = morphological::erosion(&gabarit, img_threshold);
    img_erodee.save("images_res/bureau_erodee.png").expect("Failed to save image");
}