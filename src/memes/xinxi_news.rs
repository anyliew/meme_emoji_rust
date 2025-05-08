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

fn xinxi_news(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let name = &images[0].name;
    let text = format!("{name}");
    let frame = load_image("xinxi_news/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(235, 615, 478, 659),
        &text,
        20.0,
        100.0,
        None,
    )?;
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let img = images[0].circle().resize_exact((300, 300));
        canvas.draw_image(&img, (210, 270), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "xinxi_news",
    xinxi_news,
    min_images = 1,
    max_images = 1,
    keywords = &["新喜报"],
    date_created = local_date(2024, 7, 26),
    date_modified = local_date(2024, 7, 26),
);
