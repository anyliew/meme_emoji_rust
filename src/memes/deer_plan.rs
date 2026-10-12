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

fn deer_plan(
    images: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let name = texts.first().unwrap_or(&images[0].name);
    let text = format!("{}の鹿管计划", name);
    let frame = load_image("deer_plan/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    
    // 先绘制文字
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(160, 18, 1041, 118),
        &text,
        15.0,
        120.0,
        text_params!(
            font_families = &["FZXS14"],
            text_align = TextAlign::Left,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;

    // 然后在同一个 canvas 上绘制图片
    let img = images[0].image.circle().resize_exact((100, 100));
    canvas.draw_image(&img, (35, 18), None);
    
    encode_png(surface.image_snapshot())
}

register_meme!(
    "deer_plan",
    deer_plan,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["鹿管计划"],
    date_created = local_date(2025, 9, 28),
    date_modified = local_date(2025, 9, 28),
);