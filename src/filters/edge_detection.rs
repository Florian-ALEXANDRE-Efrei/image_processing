use image::{ImageBuffer, DynamicImage, GenericImageView};

#[allow(dead_code)]
// Fonction pour calculer le gradient pour un seul masque
fn gradient(mask: [[i8; 3]; 3], pixels: [[u8; 3]; 3], normalisation: i16) -> u8 {
    let mut grad: i16 = 0;

    // Appliquer le masque aux pixels
    for i in 0..3 {
        for j in 0..3 {
            grad += mask[i][j] as i16 * pixels[i][j] as i16;
        }
    }

    // Prendre la valeur absolue et normaliser le gradient
    grad = grad.abs() / normalisation;
    grad as u8
}

// Fonction générique pour appliquer un ensemble de masques de convolution
#[allow(dead_code)]
fn apply_mask_set(masks: &Vec<[[i8; 3]; 3]>, pixel_matrix: [[u8; 3]; 3], normalisation: i16) -> u8 {
    masks.iter()
        .map(|mask| gradient(*mask, pixel_matrix, normalisation))
        .max()
        .unwrap_or(0)
    
}   

// Fonction principale pour les filtres de Kirsch
#[allow(dead_code)]
pub fn kirsch(mut img: DynamicImage) -> DynamicImage {
    img = img.grayscale();
    let (width, height) = img.dimensions();
    let mut new_img: image::RgbaImage = ImageBuffer::new(width, height);

    // Définition des masques de Kirsch
    let masks = vec![
        [[-3, -3, 5],
         [-3,  0, 5],
         [-3, -3, 5]],    // Horizontal

        [[-3,  5,  5],
         [-3,  0,  5],
         [-3, -3, -3]],    // Diagonale 45°

        [[-3, -3, -3],
         [-3,  0, -3],
         [ 5,  5,  5]],    // Vertical

        [[-3, -3, -3],
         [-3,  0,  5],
         [-3,  5,  5]],    // Diagonale -45°
    ];

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            // Extraction de la matrice de pixels
            let pixel_matrix = [
                [img.get_pixel(x-1, y-1)[0], img.get_pixel(x, y-1)[0], img.get_pixel(x+1, y-1)[0]],
                [img.get_pixel(x-1, y)[0],   img.get_pixel(x, y)[0],   img.get_pixel(x+1, y)[0]],
                [img.get_pixel(x-1, y+1)[0], img.get_pixel(x, y+1)[0], img.get_pixel(x+1, y+1)[0]],
            ];

            // Facteur de normalisation = 1/15 (correspond à la somme des coefficients positifs ou négatifs)
            let gradient = apply_mask_set(&masks, pixel_matrix, 15);
            new_img.put_pixel(x, y, image::Rgba([gradient, gradient, gradient, 255]));
        }
        println!("kirsch: {}%", (100 * y) / height + 1);
    }
    image::DynamicImage::ImageRgba8(new_img)
}

// Fonction principale pour le filtre de Sobel
#[allow(dead_code)]
pub fn sobel(mut img: DynamicImage) -> DynamicImage {
    img = img.grayscale();
    let (width, height) = img.dimensions();
    let mut new_img: image::RgbaImage = ImageBuffer::new(width, height);

    // Définition des masques de Sobel
    let masks = vec![
        [[-1, 0, 1], 
         [-2, 0, 2], 
         [-1, 0, 1]],  // Horizontal
        
        [[-1, -2, -1],
         [ 0,  0,  0], 
         [ 1,  2,  1]],  // Vertical
    ];

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let pixel_matrix = [
                [img.get_pixel(x-1, y-1)[0], img.get_pixel(x, y-1)[0], img.get_pixel(x+1, y-1)[0]],
                [img.get_pixel(x-1, y)[0],   img.get_pixel(x, y)[0],   img.get_pixel(x+1, y)[0]],
                [img.get_pixel(x-1, y+1)[0], img.get_pixel(x, y+1)[0], img.get_pixel(x+1, y+1)[0]],
            ];

            // Facteur de normalisation = 1/4 (correspond à la somme des coefficients positifs ou négatifs)
            let gradient = apply_mask_set(&masks, pixel_matrix, 4);
            new_img.put_pixel(x, y, image::Rgba([gradient, gradient, gradient, 255]));
        }
        println!("Sobel: {}%", (100 * y) / height + 1);
    }
    image::DynamicImage::ImageRgba8(new_img)
}

// Fonction principale pour le filtre de Prewitt
#[allow(dead_code)]
pub fn prewitt(mut img: DynamicImage) -> DynamicImage {
    img = img.grayscale();
    let (width, height) = img.dimensions();
    let mut new_img: image::RgbaImage = ImageBuffer::new(width, height);

    // Définition des masques de Prewitt
    let masks = vec![
        [[-1, 0, 1],
         [-1, 0, 1],
         [-1, 0, 1]],  // Horizontal
        
        [[-1, -1, -1],
         [ 0,  0,  0],
         [ 1,  1,  1]],  // Vertical
    ];

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let pixel_matrix = [
                [img.get_pixel(x-1, y-1)[0], img.get_pixel(x, y-1)[0], img.get_pixel(x+1, y-1)[0]],
                [img.get_pixel(x-1, y)[0],   img.get_pixel(x, y)[0],   img.get_pixel(x+1, y)[0]],
                [img.get_pixel(x-1, y+1)[0], img.get_pixel(x, y+1)[0], img.get_pixel(x+1, y+1)[0]],
            ];

            // Facteur de normalisation = 1/3 (correspond à la somme des coefficients positifs ou négatifs)
            let gradient = apply_mask_set(&masks, pixel_matrix, 3);
            new_img.put_pixel(x, y, image::Rgba([gradient, gradient, gradient, 255]));
        }
        println!("Prewitt: {}%", (100 * y) / height + 1);
    }
    image::DynamicImage::ImageRgba8(new_img)
}

// Fonction principale pour les filtres de Robinson
#[allow(dead_code)]
pub fn robinson(mut img: DynamicImage) -> DynamicImage {
    img = img.grayscale();
    let (width, height) = img.dimensions();
    let mut new_img: image::RgbaImage = ImageBuffer::new(width, height);

    // Définition des masques de Robinson
    let masks = vec![
        [[ 1,   1,  1], 
         [ 1,  -2,  1], 
         [-1,  -1, -1]],    // N

        [[ 1,  1,  1], 
         [-1, -2,  1], 
         [-1, -1,  1]],    // NE

        [[-1,  1, 1], 
         [-1, -2, 1], 
         [-1,  1, 1]],    // E

        [[-1, -1, 1], 
         [-1, -2, 1], 
         [ 1,  1, 1]],    // SE
        
        [[-1, -1, -1],
         [ 1, -2,  1],
         [ 1,  1,  1]],    // S

        [[1, -1, -1],
         [1, -2, -1],
         [1,  1,  1]],    // SO

        [[1,  1, -1],
         [1, -2, -1],
         [1,  1, -1]],    // O

        [[1,  1,  1],
         [1, -2, -1],
         [1, -1, -1]],    // NO
    ];

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            // Extraction de la matrice de pixels
            let pixel_matrix = [
                [img.get_pixel(x-1, y-1)[0], img.get_pixel(x, y-1)[0], img.get_pixel(x+1, y-1)[0]],
                [img.get_pixel(x-1, y)[0],   img.get_pixel(x, y)[0],   img.get_pixel(x+1, y)[0]],
                [img.get_pixel(x-1, y+1)[0], img.get_pixel(x, y+1)[0], img.get_pixel(x+1, y+1)[0]],
            ];

            // Facteur de normalisation = 1/5 (correspond à la somme des coefficients positifs ou négatifs)
            let gradient = apply_mask_set(&masks, pixel_matrix, 5);
            new_img.put_pixel(x, y, image::Rgba([gradient, gradient, gradient, 255]));
        }
        println!("Robinson: {}%", (100 * y) / height + 1);
    }
    image::DynamicImage::ImageRgba8(new_img)
}
