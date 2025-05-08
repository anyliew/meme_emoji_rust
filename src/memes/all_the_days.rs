use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn all_the_days(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("all_the_days/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let image1 = images[0].resize_fit((95, 95), Fit::Cover);
        let image2 = images[1].resize_fit((95, 95), Fit::Cover);
        canvas.draw_image(&image1, (255, 255), None);
        canvas.draw_image(&image2, (425, 210), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "all_the_days",
    all_the_days,
    min_images = 2,
    max_images = 2,
    keywords = &["一生一世"],
    date_created = local_date(2023, 10, 3),
    date_modified = local_date(2023, 10, 3),
);