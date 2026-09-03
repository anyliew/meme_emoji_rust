use rand::RngExt;
use skia_safe::IRect;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{color_from_hex_code, load_image, local_date, new_paint, new_surface},
};

use crate::{options::number_option, register_meme, tags::MemeTags};

number_option!(Number, 1, 5);

fn kurogames_changli_holdsign(
    _: Vec<InputImage>,
    texts: Vec<String>,
    options: Number,
) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let num = options.number.unwrap_or(rand::rng().random_range(1..=5));
    let params = [
        ((500, 200), (262, 820), 0.0),
        ((500, 200), (242, 760), 8.0),
        ((500, 200), (252, 760), 0.0),
        ((500, 200), (252, 760), 0.0),
        ((500, 200), (252, 800), 8.0),
    ];
    let (size, loc, angle) = params[num as usize - 1];

    let mut text_surface = new_surface(size);
    let canvas = text_surface.canvas();
    let padding = 10;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(padding, padding, size.0 - padding, size.1 - padding),
        text,
        60.0,
        120.0,
        text_params!(
            font_families = &["FZShaoEr-M11S"],
            paint = new_paint(color_from_hex_code("#3b0b07")),
        ),
    )?;
    let mut text_image = text_surface.image_snapshot();
    if angle != 0.0 {
        text_image = text_image.rotate(angle);
    }

    let frame = load_image(format!("kurogames_changli_holdsign/changli_{num:02}.png"))?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_image(&text_image, loc, None);
    encode_png(surface.image_snapshot())
}

register_meme!(
    "kurogames_changli_holdsign",
    kurogames_changli_holdsign,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["弈棋布势之道，如同万物运转"],
    keywords = &["长离举牌"],
    tags = MemeTags::changli(),
    date_created = local_date(2025, 8, 25),
    date_modified = local_date(2025, 8, 25),
);
