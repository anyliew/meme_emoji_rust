use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn mi_monkey(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("mi_monkey/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].resize_fit((180, 180), Fit::Cover).circle();
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_image(&image, (500, 100), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "mi_monkey",
    mi_monkey,
    min_images = 1,
    max_images = 1,
    keywords = &["米猴", "🐒", "🐵"],
    date_created = local_date(2024, 7, 26),
    date_modified = local_date(2026, 10, 12),
);
