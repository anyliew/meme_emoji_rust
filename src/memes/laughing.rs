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

use crate::{options::NoOptions, register_meme};

fn laughing(
    images: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let name = texts.first().unwrap_or(&images[0].name);
    let text = format!("{},你真是好搞笑呀🤣😂~", name);
    let frame = load_image("laughing/0.jpg")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    
    // 先绘制文字
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(1, 1, 1495, 407),
        &text,
        20.0,
        100.0,
        text_params!(
            font_families = &["FZXS14"],
            text_align = TextAlign::Left,
            paint = new_paint(Color::from_rgb(255, 255, 255)),
        ),
    )?;

    // 然后在同一个 canvas 上绘制图片
    let img = images[0].image.circle().resize_exact((400, 400));
    canvas.draw_image(&img, (1097, 550), None);
    
    encode_png(surface.image_snapshot())
}

register_meme!(
    "laughing",
    laughing,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["笑指"],
    date_created = local_date(2025, 7, 2),
    date_modified = local_date(2025, 7, 2),
);