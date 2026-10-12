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

fn kurogames_songlun_finger(
    images: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let name = texts.first().unwrap_or(&images[0].name);
    let text = format!("难道说\n\n{}是潮批?", name);
    let frame = load_image("kurogames_songlun_finger/0.jpg")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    
    // 先绘制文字
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(3, 350, 482, 606),
        &text,
        20.0,
        100.0,
        text_params!(
            font_families = &["FZShaoEr-M11S"],
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;

    // 然后在同一个 canvas 上绘制图片
    let img = images[0].image.circle().resize_exact((300, 300));
    canvas.draw_image(&img, (100, 40), None);
    
    encode_png(surface.image_snapshot())
}

register_meme!(
    "kurogames_songlun_finger",
    kurogames_songlun_finger,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    tags = MemeTags::wuthering_waves(),
    keywords = &["松伦指","潮批"],
    date_created = local_date(2025, 7, 1),
    date_modified = local_date(2026, 7, 21),
);