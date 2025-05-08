use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn aircraft_cup_cleaning_liquid(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("aircraft_cup_cleaning_liquid/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].resize_fit((920, 920), Fit::Cover);
        canvas.draw_image(&image, (290, 20), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "aircraft_cup_cleaning_liquid",
    aircraft_cup_cleaning_liquid,
    min_images = 1,
    max_images = 1,
    keywords = &["清洗液"],
    date_created = local_date(2022, 4, 14),
    date_modified = local_date(2023, 2, 14),
);