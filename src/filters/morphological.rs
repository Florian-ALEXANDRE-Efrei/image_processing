use image::{DynamicImage, ImageBuffer, GenericImageView};
use crate::utils::image_utils;

#[allow(dead_code)]
pub fn erosion<T: std::cmp::PartialEq<image::Rgba<u8>>, const M: usize, const N: usize>(gabarit: &[[T; N]; M], img : DynamicImage) -> DynamicImage
{
    let gabarit_taille = gabarit.len() as u32;
    let gabarit_centre = gabarit_taille / 2;

    let (width, height) = img.dimensions();
    let mut img_erodee: image::RgbaImage = ImageBuffer::new(width, height); // WIDTH, HEIGHT

    for y in gabarit_centre..height-gabarit_centre
    {
        for x in gabarit_centre..width-gabarit_centre
        {
            let mut match_gabarit = true;
            // Vérification des pixels voisins avec le gabarit
            for ky in 0..gabarit_taille {
                for kx in 0..gabarit_taille {

                    // mise à jours des coordonnées des pixels de l'image
                    let nx = x + kx - gabarit_centre;
                    let ny = y + ky - gabarit_centre;

                    // récupération du pixel de l'image équivalent au gabarit
                    let pixel = img.get_pixel(nx, ny);

                    // tous les pixels objets du gabarit doivent treouver leurs équivalent dans les pixels voisins du pixel courant.
                    if pixel == image_utils::WHITEPIXEL && gabarit[ky as usize][kx as usize] == image_utils::BLACKPIXEL
                    {
                        match_gabarit = false;
                        break;
                    }
                }
            }
            // Application du pixel érodé
            if match_gabarit // tous les pixels objets du gabarit ont trouvé leurs équivalent.
            {
                img_erodee.put_pixel(x, y, image_utils::BLACKPIXEL);
            } else // au moins un pixel objet du gabarit n'a pas trouvé d'équivalent.
            {
                img_erodee.put_pixel(x, y, image_utils::WHITEPIXEL);
            }
        }
    }
    return image::DynamicImage::ImageRgba8(img_erodee);
}

#[allow(dead_code)]
pub fn dilatation<T: std::cmp::PartialEq<image::Rgba<u8>>, const M: usize, const N: usize>(gabarit: &[[T; N]; M], img : DynamicImage) -> DynamicImage
{
    let gabarit_taille = gabarit.len() as u32;
    let gabarit_centre = gabarit_taille / 2;

    let (width, height) = img.dimensions();
    let mut img_dilatee: image::RgbaImage = ImageBuffer::new(width, height); // WIDTH, HEIGHT

    for y in gabarit_centre..height-gabarit_centre
    {
        for x in gabarit_centre..width-gabarit_centre
        {
            let mut match_gabarit = false;
            // Vérification des pixels voisins avec le gabarit
            for ky in 0..gabarit_taille
            {
                for kx in 0..gabarit_taille
                {
                    // mise à jours des coordonnées des pixels de l'image
                    let nx = x + kx - gabarit_centre;
                    let ny = y + ky - gabarit_centre;

                    // récupération du pixel de l'image équivalent au gabarit
                    let pixel = img.get_pixel(nx, ny);

                    // Si au moins un pixel objet du gabarit correspond à un pixel objet du pixel courant et de son voisinage alors, le pixel central devient un pixel objet.
                    if pixel == image_utils::BLACKPIXEL && gabarit[ky as usize][kx as usize] == image_utils::BLACKPIXEL
                    {
                        match_gabarit = true;
                        break;
                    }
                }
            }
            // Application du pixel érodé
            if match_gabarit // au moins un pixel objet du gabarit à trouvé son équivalent
            {
                img_dilatee.put_pixel(x, y, image_utils::BLACKPIXEL);
            } else // aucun pixel objet du gabarit n'a trouvé d'équivalent
            {
                img_dilatee.put_pixel(x, y, image_utils::WHITEPIXEL);
            }
        }
    }
    return image::DynamicImage::ImageRgba8(img_dilatee);
}