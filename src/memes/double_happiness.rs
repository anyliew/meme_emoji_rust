use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn double_happiness(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("double_happiness/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let img1 = images[1].circle().resize_exact((522, 522));
        let img0 = images[0].circle().resize_exact((536, 536));
        canvas.draw_image(&img1, (1206, 138), None);
        canvas.draw_image(&img0, (532, 332), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "double_happiness",
    double_happiness,
    min_images = 2,
    max_images = 2,
    keywords = &["双喜"],
    date_created = local_date(2026, 2, 16),
    date_modified = local_date(2026, 2, 16),
);
