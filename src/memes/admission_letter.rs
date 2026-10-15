use chrono::{Datelike, Local};

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

use crate::register_meme;

const DEFAULT_TEXT: &str = "KaedeharaLu";
const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

#[derive(MemeOptions)]
pub(crate) struct AdmissionLetterOptions {
    #[option(short, long, description = "昵称（第一段文字）")]
    name: Option<String>,
}

fn admission_letter(
    _: Vec<InputImage>,
    texts: Vec<String>,
    options: AdmissionLetterOptions,
) -> Result<Vec<u8>, Error> {
    let name = options
        .name
        .clone()
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| {
            texts
                .first()
                .cloned()
                .unwrap_or_else(|| DEFAULT_TEXT.to_string())
        });
    let now = Local::now();
    let date_str = format!(
        "{} {} {}",
        MONTHS[(now.month() - 1) as usize],
        now.day(),
        now.year()
    );
    let text = format!("{date_str}\n\nDear {name}");

    let frame = load_image("admission_letter/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(60, 225, 490, 320),
        &text,
        10.0,
        24.0,
        text_params!(
            font_families = &["FZShaoEr-M11S"],
            text_align = TextAlign::Left,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "admission_letter",
    admission_letter,
    min_images = 0,
    max_images = 0,
    min_texts = 1,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["录取通知书"],
    date_created = local_date(2025, 8, 25),
    date_modified = local_date(2026, 10, 14),
);
