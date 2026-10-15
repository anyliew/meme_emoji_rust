use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn kurogames_phoebe_love(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("kurogames_phoebe_love/0.png")?;

    let func = |images: Vec<Image>| {
        let img = images[0].resize_fit((439, 312), Fit::Cover);
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&img, (156, 688), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "kurogames_phoebe_love",
    kurogames_phoebe_love,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 0,
    tags = MemeTags::wuthering_waves(),
    keywords = &["菲比爱心"],
    date_created = local_date(2026, 10, 14),
    date_modified = local_date(2026, 10, 14),
);
