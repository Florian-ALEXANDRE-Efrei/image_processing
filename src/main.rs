use image_processing::filters::{edge_detection, morphological};
use image_processing::utils::image_utils;

fn main() {
    let img = image_utils::get_image("images_src/Bureau.png"); // Ouverture de l'image

    let img_prewitt = edge_detection::prewitt(img.clone()); // Application du filtre de Prewitt
    img_prewitt.save("images_res/bureau_prewitt.png").expect("Failed to save image"); // Enregistrement de l'image résultante

    let img_sobel = edge_detection::sobel(img.clone()); // Application du filtre de Sobel
    img_sobel.save("images_res/bureau_sobel.png").expect("Failed to save image"); // Enregistrement de l'image résultante

    let img_kirsch = edge_detection::kirsch(img.clone()); // Application du filtre de Kirsch
    img_kirsch.save("images_res/bureau_kirsch.png").expect("Failed to save image"); // Enregistrement de l'image résultante

    let img_robinson = edge_detection::robinson(img.clone()); // Application du filtre de Robinson
    img_robinson.save("images_res/bureau_robinson.png").expect("Failed to save image"); // Enregistrement de l'image résultante

    let img_threshold = image_utils::thresholding(img_prewitt, 30); // Application du seuillage classique avec comme seuil 30
    img_threshold.save("images_res/bureau_thresholding.png").expect("Failed to save image"); // Enregistrement de l'image résultante

    let gabarit = [ // création du gabarit pour la dilatation et l'érosion
        [image_utils::WHITEPIXEL, image_utils::BLACKPIXEL, image_utils::WHITEPIXEL],
        [image_utils::BLACKPIXEL, image_utils::BLACKPIXEL, image_utils::BLACKPIXEL],
        [image_utils::WHITEPIXEL, image_utils::BLACKPIXEL, image_utils::WHITEPIXEL],
    ];

    let img_dilatee = morphological::dilatation(&gabarit, img_threshold.clone()); // Application de la dilatation
    img_dilatee.save("images_res/bureau_dilatee.png").expect("Failed to save image"); // Enregistrement de l'image résultante

    let img_erodee = morphological::erosion(&gabarit, img_threshold); // Application de l'érosion
    img_erodee.save("images_res/bureau_erodee.png").expect("Failed to save image"); // Enregistrement de l'image résultante
}