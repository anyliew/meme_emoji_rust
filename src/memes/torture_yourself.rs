use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn torture_yourself(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("torture_yourself/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let image1 = images[0].resize_fit((400, 400), Fit::Cover);
        let image2 = images[1].resize_fit((200, 200), Fit::Cover);
        canvas.draw_image(&image1, (70, 283), None);
        canvas.draw_image(&image2, (725, 780), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "torture_yourself",
    torture_yourself,
    min_images = 2,
    max_images = 2,
    keywords = &["折磨自己"],
    date_created = local_date(2023, 10, 3),
    date_modified = local_date(2023, 10, 3),
);