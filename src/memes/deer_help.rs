use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn deer_help(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("deer_help/0.jpg")?;

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let image1 = images[0].circle().resize_fit((180, 180), Fit::Cover);
        let image2 = images[1].circle().resize_fit((180, 180), Fit::Cover);
        canvas.draw_image(&image1, (300, 190), None);
        canvas.draw_image(&image2, (605, 200), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "deer_help",
    deer_help,
    min_images = 2,
    max_images = 2,
    keywords = &["帮鹿","帮🦌"],
    date_created = local_date(2023, 10, 3),
    date_modified = local_date(2023, 10, 3),
);