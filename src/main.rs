mod tools;
mod operateur;
mod detection_contour;
mod dev_file;

fn main() {

    let mut img = tools::get_image("../images/mascaron.png");
    img = detection_contour::robinson(img);
    img = tools::thresholding(img, 10);
    img.save("../result_images/mascaron_robinson.png").expect("Failed to save image");

    let gabarit = [
        [tools::WHITEPIXEL, tools::BLACKPIXEL, tools::WHITEPIXEL],
        [tools::BLACKPIXEL, tools::BLACKPIXEL, tools::BLACKPIXEL],
        [tools::WHITEPIXEL, tools::BLACKPIXEL, tools::WHITEPIXEL],
    ];

    let mut img = tools::get_image("../result_images/mascaron_robinson.png");
    img = operateur::erosion(&gabarit, img);
    img.save("../result_images/mascaron_robinson_erodee.png").expect("Failed to save image");

    let mut img = tools::get_image("../result_images/mascaron_robinson.png");
    img = operateur::dilatation(&gabarit, img);
    img.save("../result_images/mascaron_robinson_dilatee.png").expect("Failed to save image");
}