use image::{GenericImageView, DynamicImage, GenericImage, ImageBuffer};

mod sobelfilter;
mod threshold;

// ┌──────────── Y
// │    
// │    image
// │
// │
// X

#[allow(dead_code)]
fn get_image(dir: &str) -> DynamicImage
{
    let img = image::open(dir).expect("Failed to open image");
    return img;
}

#[allow(dead_code)]
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


#[allow(dead_code)]
fn png_to_ascii(img: DynamicImage, scale: u32)
{
    let ascii_bis = ["$","@","B","%","8","&","W","M","#","*","o","a","h","k","b","d","p","q","w","m","Z","O","0","Q","L","C","J","U","Y","X","z","c","v","u","n","x","r","j","f","t","/","\\","|","(",")","1","{","}","[","]","?","-","_","+","~","<",">","i","!","l","I",";",":",",","^","`","'","."," "];
    let mut m2 = ascii_bis;
    m2.reverse();
    //let ascii_online = " `.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";
    //let ascii_bis = [".", ",", ":", ";", "'", "-", "_", "|", "(", ")", "~", "+", "^", "/", "!", "=", "<", ">", "*", "i", "l", "t", "r", "j", "1", "f", "c", "s", "u", "x", "z", "o", "a", "e", "n", "m", "k", "p", "g", "q", "v", "y", "2", "3", "4", "5", "6", "7", "8", "9", "0", "w", "B", "D", "H", "K", "M", "O", "Q", "R", "U", "V", "W", "X", "Z"];
    //println!("size {}", ascii_online.len());
    //let ascii = [" ", ".", ",", "-", "~", "~", "+", "+", "=", "=", "@", "@"];
    //let ascii = ["@", "=", "+", "~", "-", ",", ".", " "];
    let (width, height) = img.dimensions();
    for y in 0..height
    {
        for x in 0..width
        {
            if y % (scale*2) == 0 && x % scale == 0
            {
                let pixel = img.get_pixel(x, y);
                let intent: f32 = (pixel[0]/3 + pixel[1]/3 + pixel[2]/3).into();
                //index = intent as f32/2.79;
                let index : f32 = intent /3.7;
                //print!("{}", ascii_online.chars().nth(index as usize).unwrap());
                print!("{}", m2[index as usize]);
            }
            
        }
        if y % (scale*2) == 0 
        {
            println!("");
        }
    }
}

#[allow(dead_code)]
fn crop_image(img: DynamicImage) -> DynamicImage
{
    let (width, height) = img.dimensions();
    let mut new_img: image::RgbaImage = ImageBuffer::new(15, 20);
    for y in 0..height
    {
        for x in 0..width
        {
            let pixel = img.get_pixel(x, y);
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

#[allow(dead_code)]
fn png_to_typewriter(img0: DynamicImage, img1: DynamicImage) -> DynamicImage
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
                        let pixel = img0.get_pixel(x_img0, y_img0);
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


#[allow(dead_code)]
fn erode_image(img: DynamicImage, kernel_size: u32) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut eroded_img = DynamicImage::new_rgba8(width, height);
    let kernel_radius = kernel_size / 2;

    for y in 0..height {
        for x in 0..width {
            let mut min_pixel = Rgba([255, 255, 255, 255]);
            for ky in 0..kernel_size {
                for kx in 0..kernel_size {
                    let nx = x + kx - kernel_radius;
                    let ny = y + ky - kernel_radius;
                    if nx < width && ny < height {
                        let pixel = img.get_pixel(nx, ny);
                        for i in 0..3 {
                            if pixel[i] < min_pixel[i] {
                                min_pixel[i] = pixel[i];
                            }
                        }
                    }
                }
            }
            eroded_img.put_pixel(x, y, min_pixel);
        }
    }
    eroded_img
}


fn main() {

    let mut img = get_image("../images/immeubles_haussmanniens.png");
    //img = invert_color(img);
    //img = sobelfilter::filtre_de_sobel(img);
    img = threshold::multi_thresholding(img, 2);
    //img = threshold::multiple_hat_thresholding(img);
    //img = threshold::thresholding(img, 150);

    //png_to_ascii(img, 2);

    img.save("../result_images/multiple_thresholding_res.png").expect("Failed to save image");
}

