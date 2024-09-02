#[warn(unused_mut)] 

use image::{GenericImageView, DynamicImage, Rgba, Rgb, GenericImage, ImageBuffer, Pixel};

//const WHITE_PIXEL: dyn image::Pixel = image::Rgb([255, 255, 255]);
//const BLACK_PIXEL: dyn image::Pixel = image::Rgb([0, 0, 0]);
const CLEARPIXEL: image::Rgba::<u8> = image::Rgba::<u8>([0,0,0,0]);
const WHITEPIXEL: image::Rgba::<u8> = image::Rgba::<u8>([255,255,255,255]);
const BLACKPIXEL: image::Rgba::<u8> = image::Rgba::<u8>([0,0,0,255]);


// ┌──────────── Y
// │    
// │    image
// │
// │
// X


fn get_image(dir: &str) -> DynamicImage
{
    let img = image::open(dir).expect("Failed to open image");
    return img;
}

fn invert_color(mut img: DynamicImage) -> DynamicImage
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

fn thresholding(mut img: DynamicImage, threshold: u8) -> DynamicImage
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

            if pixel.to_rgba()[0] < threshold
            {
                img.put_pixel(x, y, BLACKPIXEL);
            }
            else 
            {
                img.put_pixel(x, y, WHITEPIXEL);
            }               
        }
    }
    
    
    return img;
}


fn png_to_ascii(img: DynamicImage, scale: u32)
{
    let ascii_bis = [".", ",", ":", ";", "'", "\"", "-", "_", "|", "(", ")", "~", "+", "^", "\\", "/", "!", "=", "<", ">", "*", "i", "l", "t", "r", "j", "1", "f", "c", "s", "u", "x", "z", "o", "a", "e", "n", "m", "k", "p", "g", "q", "v", "y", "2", "3", "4", "5", "6", "7", "8", "9", "0", "w", "B", "D", "H", "K", "M", "O", "Q", "R", "U", "V", "W", "X", "Z"];
    println!("size {}", ascii_bis.len());
    //let ascii = [" ", ".", ",", "-", "~", "+", "=", "@"];
    let ascii = ["@", "=", "+", "~", "-", ",", ".", " "];
    let mut index : u8 = 0;
    let (width, height) = img.dimensions();
    for y in 0..height
    {
        for x in 0..width
        {
            if y % (scale*2) == 0 && x % scale == 0
            {
                let pixel = img.get_pixel(x, y);
                let mut intent = pixel[0]/3 + pixel[1]/3 + pixel[2]/3;
                index = intent/32;
                print!("{}", ascii[index as usize]);
            }
            
        }
        if y % (scale*2) == 0 
        {
            println!("");
        }
    }
}



fn crop_image(mut img: DynamicImage) -> DynamicImage
{
    let (width, height) = img.dimensions();
    let mut new_img: image::RgbaImage = ImageBuffer::new(15, 20);
    for y in 0..height
    {
        for x in 0..width
        {
            let mut pixel = img.get_pixel(x, y);
            // on ne traites que les pixels qui ne sont pas transparents 
            if pixel[3] != 0
            {
                // on garde que les pixels qui ont un rouge inferieur à 230 (rgb: 255, 255, 255 => blanc)
                if pixel[1] < 230
                {
                    new_img.put_pixel(x-6, y-41, pixel);
                    println!("coord: ({}, {}), color: {:?}", x, y, pixel);
                }
                    
            }
        }
    }
    return image::DynamicImage::ImageRgba8(new_img);
}

fn png_to_typescripte(mut img0: DynamicImage, mut img1: DynamicImage) -> DynamicImage
{
    println!("dim img0: {:?}, img1: {:?}",img0.dimensions(),img1.dimensions());
    //let (width, height) = img.dimensions();
    let (width_img_0, height_img_0) = img0.dimensions();
    let mut new_img: image::RgbaImage = ImageBuffer::new(width_img_0*3, height_img_0*3); // WIDTH, HEIGHT
        
    for y in 0..100
    {
        for x in 0..100
        {
            if x % height_img_0 == 0 && width_img_0 % 19 == 0 
            {
                for y_img0 in 0..height_img_0
                {
                    for x_img0 in 0..width_img_0 
                    {
                        let mut pixel = img0.get_pixel(x_img0, y_img0);
                        if pixel[3] != 0 
                        {
                            if x + x_img0 < 14*3 && y + y_img0 < 19*3 
                            {
                                new_img.put_pixel(x+x_img0, y+y_img0, pixel);
                            }
                        }
                    }
                }
            }
            //let mut pixel = img.get_pixel(x, y);
            // if x % 4 == 0 && y % 4 == 0 
            // {
            //     let mut pixel = img.get_pixel(x, y);
            //     img.put_pixel(x, y, WHITEPIXEL);
            // }       
        }
    }
    return image::DynamicImage::ImageRgba8(new_img);
    

}

fn main() {
    let mut img = get_image("../images/profile_picture.png");
    //let mut img0 = get_image("images/0_crop.png");
    //let mut img1 = get_image("images/1_crop.png");


    //img = invert_color(img);
    img = thresholding(img, 120);
    //let mut img = png_to_typescripte(img0, img1);
    //png_to_ascii(img, 8);
    img.save("../result_images/output.png").expect("Failed to save image");
}

