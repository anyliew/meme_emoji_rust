use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn kurogames_songlun(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("kurogames_songlun/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].resize_fit((180, 180), Fit::Cover);
        canvas.draw_image(&image, (500, 100), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "kurogames_songlun",
    kurogames_songlun,
    min_images = 1,
    max_images = 1,
    keywords = &["松伦哥指", "潮批"],
    date_created = local_date(2022, 4, 14),
    date_modified = local_date(2023, 2, 14),
);