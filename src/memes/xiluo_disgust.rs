use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn xiluo_disgust(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("xiluo_disgust/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let img = images[0].resize_fit((500, 800), Fit::Cover).rotate(8.0);
        canvas.draw_image(&img, (0, 75), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "xiluo_disgust",
    xiluo_disgust,
    min_images = 1,
    max_images = 1,
    keywords = &["希罗嫌弃", "二阶堂希罗嫌弃"],
    date_created = local_date(2025, 10, 5),
    date_modified = local_date(2025, 10, 5),
);
