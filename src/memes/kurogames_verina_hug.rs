use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn kurogames_verina_hug(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("kurogames_verina_hug/0.png")?;

    let func = |images: Vec<Image>| {
        let img = images[0].resize_fit((413, 305), Fit::Cover);
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&img, (222, 742), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "kurogames_verina_hug",
    kurogames_verina_hug,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 0,
    tags = MemeTags::wuthering_waves(),
    keywords = &["维里奈抱", "小维抱"],
    date_created = local_date(2026, 10, 14),
    date_modified = local_date(2026, 10, 14),
);
