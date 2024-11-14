use image::{DynamicImage, GenericImageView, GenericImage};

#[allow(dead_code)]
pub const CLEARPIXEL: image::Rgba::<u8> = image::Rgba::<u8>([0,0,0,0]);

#[allow(dead_code)]
pub const WHITEPIXEL: image::Rgba::<u8> = image::Rgba::<u8>([255,255,255,255]);

#[allow(dead_code)]
pub const BLACKPIXEL: image::Rgba::<u8> = image::Rgba::<u8>([0,0,0,255]);

#[allow(dead_code)]
pub fn get_image(dir: &str) -> DynamicImage
{
    let img = image::open(dir).expect("Failed to open image");
    return img;
}

#[allow(dead_code)]
pub fn thresholding(mut img: DynamicImage, threshold: u8) -> DynamicImage
// threshold à un type u8 car 8 bit est la taille d'une couleur (entre 0 et 255)
{
    // Convertion de l'image en nuance de gris
    img = img.grayscale();

    // Récupération de la dimsension
    let (width, height) = img.dimensions();

    // Parcours de chaque pixel de gauche à droite de haut en bas.
    for y in 0..height
    {
        for x in 0..width
        {
            let pixel = img.get_pixel(x, y);

            if pixel[0] < threshold
            {
                img.put_pixel(x, y, WHITEPIXEL);
            }
            else
            {
                img.put_pixel(x, y, BLACKPIXEL);
            }
        }
    }

    return img;
}

#[allow(dead_code)]
pub fn invert_color(mut img: DynamicImage) -> DynamicImage
{
    let (width, height) = img.dimensions();
    for y in 0..height
    {
        for x in 0..width
        {
            let mut pixel = img.get_pixel(x, y);
            pixel = image::Rgba([255 - pixel[0], 255 - pixel[1], 255 - pixel[2], pixel[3]]);
            img.put_pixel(x, y, pixel);
        }
    }
    return img;
}