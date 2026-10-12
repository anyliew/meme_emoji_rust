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

number_option!(Number, 0, 2);

const DEFAULT_TEXT: &str = "我想留在大家身边……从过去，一同迈向明天";

fn kurogames_rover_say(
    _: Vec<InputImage>,
    texts: Vec<String>,
    options: Number,
) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let total_num = 2;
    let num = match options.number {
        Some(n) if n >= 1 => n,
        _ => rand::rng().random_range(1..=total_num),
    };
    let index = num as usize - 1;
    let files = ["0.png", "1.png"];
    let text_areas = [
        IRect::from_ltrb(151, 140, 627, 452),
        IRect::from_ltrb(151, 140, 627, 452),
    ];

    let frame = load_image(format!("kurogames_rover_say/{}", files[index]))?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        text_areas[index],
        text,
        50.0,
        120.0,
        text_params!(
            font_families = &["FZShaoEr-M11S"],
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(7, 38, 19)),
        ),
    )?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "kurogames_rover_say",
    kurogames_rover_say,
    min_texts = 1,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["漂泊者说", "阿漂说"],
    tags = MemeTags::wuthering_waves(),
    date_created = local_date(2026, 10, 10),
    date_modified = local_date(2026, 10, 10),
);
