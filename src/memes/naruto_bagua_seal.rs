use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn naruto_bagua_seal(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("naruto_bagua_seal/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let img = images[0].resize_exact((391, 368));
        canvas.draw_image(&img, (308, 330), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "naruto_bagua_seal",
    naruto_bagua_seal,
    min_images = 1,
    max_images = 1,
    keywords = &["八卦封印"],
    date_created = local_date(2026, 1, 9),
    date_modified = local_date(2026, 1, 9),
);
