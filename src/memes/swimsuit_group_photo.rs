use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn swimsuit_group_photo(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("swimsuit_group_photo/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let image1 = images[0].resize_fit((350, 350), Fit::Cover);
        let image2 = images[1].resize_fit((350, 350), Fit::Cover);
        canvas.draw_image(&image1, (758, 280), None);
        canvas.draw_image(&image2, (1800, 280), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "swimsuit_group_photo",
    swimsuit_group_photo,
    min_images = 2,
    max_images = 2,
    keywords = &["泳衣合影"],
    date_created = local_date(2023, 10, 3),
    date_modified = local_date(2023, 10, 3),
);