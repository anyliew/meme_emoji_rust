use skia_safe::{Color, IRect, Image, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn something(
    images: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let name = if !texts.is_empty() {
        texts[0].clone()
    } else if !images[0].name.is_empty() {
        images[0].name.clone()
    } else {
        "他".to_string()
    };
    let text = format!("这{name}是什么东西?\n竟然让人如此着迷");
    let frame = load_image("something/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(117, 0, 845, 190),
        &text,
        20.0,
        90.0,
        text_params!(
            font_families = &["FZShaoEr-M11S"],
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        let img = images[0].resize_exact((308, 308));
        canvas.draw_image(&img, (118, 471), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "something",
    something,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["什么东西"],
    date_created = local_date(2026, 1, 12),
    date_modified = local_date(2026, 1, 12),
);
