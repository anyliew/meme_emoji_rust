use skia_safe::{IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn aircraft_cup_commemorative_edition_saint_sister(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let name = &images[0].name;
    let text = format!("{name}の❤️最愛");
    let frame = load_image("aircraft_cup_commemorative_edition_saint_sister/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(40, 110, 374, 207),
        &text,
        20.0,
        100.0,
        None,
    )?;
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let img = images[0].circle().resize_exact((770, 770));
        canvas.draw_image(&img, (202, 252), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "aircraft_cup_commemorative_edition_saint_sister",
    aircraft_cup_commemorative_edition_saint_sister,
    min_images = 1,
    max_images = 1,
    keywords = &["纪念版圣修女"],
    date_created = local_date(2024, 7, 26),
    date_modified = local_date(2024, 7, 26),
);
