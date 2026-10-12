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

const DEFAULT_TEXT: &str = "云青青兮欲雨，水澹澹兮生烟……";
fn kurogames_suisui_say(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { DEFAULT_TEXT };
    let frame = load_image("kurogames_suisui_say/0.png")?;
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
            paint = new_paint(Color::from_rgb(25, 46, 31)),
        ),
    )?;
    encode_png(surface.image_snapshot())
}

register_meme!(
    "kurogames_suisui_say",
    kurogames_suisui_say,
    min_texts = 1,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["穗穗说"],
    tags = MemeTags::wuthering_waves(),
    date_created = local_date(2026, 10, 10),
    date_modified = local_date(2026, 10, 10),
);
