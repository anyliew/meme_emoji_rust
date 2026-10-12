use skia_safe::{Color, IRect, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

const DEFAULT_TEXT: &str = "同赏人间欢喜，共守……灯火长明";
fn kurogames_hsin_say(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { DEFAULT_TEXT };
    let frame = load_image("kurogames_hsin_say/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(168, 140, 620, 445),
        text,
        50.0,
        120.0,
        text_params!(
            font_families = &["FZShaoEr-M11S"],
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(8, 46, 20)),
        ),
    )?;
    encode_png(surface.image_snapshot())
}

register_meme!(
    "kurogames_hsin_say",
    kurogames_hsin_say,
    min_texts = 1,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["心月狐说", "心说"],
    tags = MemeTags::wuthering_waves(),
    date_created = local_date(2026, 10, 10),
    date_modified = local_date(2026, 10, 10),
);
