use skia_safe::{Color, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    text_params,
    tools::{load_image, local_date, new_paint, new_stroke_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

const DEFAULT_TEXT: &str = "亚托莉喜欢这个";

fn murasame_like(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        DEFAULT_TEXT
    };

    let frame = load_image("murasame_like/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(
            5,
            frame.height() - 60,
            frame.width() - 5,
            frame.height() - 10,
        ),
        text,
        20.0,
        40.0,
        text_params!(
            paint = new_paint(Color::WHITE),
            stroke_paint = new_stroke_paint(Color::BLACK, 4.0),
        ),
    )?;
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        let image = images[0].resize_fit((305, 235), Fit::Cover);
        canvas.draw_image(&image, (106, 72), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "murasame_like",
    murasame_like,
    min_images = 1,
    max_images = 1,
    keywords = &["丛雨喜欢"],
    default_texts = &[DEFAULT_TEXT],
    date_created = local_date(2022, 5, 10),
    date_modified = local_date(2023, 2, 14),
);