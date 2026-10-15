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

const DEFAULT_TEXT: &str = "真是一派荒芜的景象，让我赐予你们水的恩泽吧！";
fn mihoyo_funina_holdsign(
    _: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { DEFAULT_TEXT };
    let frame = load_image("mihoyo_funina_holdsign/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(616, 731, 925, 950),
        text,
        10.0,
        200.0,
        text_params!(
            font_families = &["FZShaoEr-M11S"],
            text_align = TextAlign::Left,
            paint = new_paint(Color::from_rgb(116, 111, 112)),
        ),
    )?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "mihoyo_funina_holdsign",
    mihoyo_funina_holdsign,
    min_texts = 1,
    max_texts = 1,
    tags = MemeTags::mihoyo(),
    default_texts = &[DEFAULT_TEXT],
    keywords = &["芙宁娜举牌","芙芙举牌","芙芙酱举牌"],
    date_created = local_date(2025, 6, 1),
    date_modified = local_date(2026, 10, 13),
);
