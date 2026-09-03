use rand::RngExt;
use skia_safe::{Color, IRect, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint},
};

use crate::{options::number_option, register_meme, tags::MemeTags};

number_option!(Number, 1, 4);

fn acacia_anan_holdsign(
    _: Vec<InputImage>,
    texts: Vec<String>,
    options: Number,
) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let num = options.number.unwrap_or(rand::rng().random_range(1..=4));
    let files = ["0.png", "1.png", "2.jpg", "3.jpg"];
    let rects = [
        IRect::from_ltrb(147, 810, 736, 1105),
        IRect::from_ltrb(179, 344, 464, 413),
        IRect::from_ltrb(150, 470, 400, 600),
        IRect::from_ltrb(310, 740, 790, 990),
    ];
    let aligns = [
        TextAlign::Center,
        TextAlign::Center,
        TextAlign::Left,
        TextAlign::Left,
    ];
    let min_sizes = [30.0, 30.0, 5.0, 5.0];
    let index = (num as usize) - 1;

    let frame = load_image(format!("acacia_anan_holdsign/{}", files[index]))?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        rects[index],
        text,
        min_sizes[index],
        120.0,
        text_params!(
            font_families = &["FZShaoEr-M11S"],
            text_align = aligns[index],
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;
    encode_png(surface.image_snapshot())
}

register_meme!(
    "acacia_anan_holdsign",
    acacia_anan_holdsign,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["快说吾辈可爱"],
    keywords = &["安安举牌", "夏目安安举牌"],
    tags = MemeTags::natsume_anan(),
    date_created = local_date(2025, 10, 27),
    date_modified = local_date(2026, 1, 12),
);
