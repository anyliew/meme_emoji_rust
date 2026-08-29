use skia_safe::{Color, IRect, Image, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn yuzu_soft_shocked(
    images: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let name = if !texts.is_empty() {
        texts[0].clone()
    } else if !images[0].name.is_empty() {
        images[0].name.clone()
    } else {
        "她".to_string()
    };
    let text = format!("{name},你是柚...柚子厨?!");
    let frame = load_image("yuzu_soft_shocked/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let img = images[0].circle().resize_exact((144, 144));
        canvas.draw_image(&img, (0, 423), None);
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(144, 423, 1080, 567),
            &text,
            20.0,
            180.0,
            text_params!(
                text_align = TextAlign::Left,
                paint = new_paint(Color::from_rgb(0, 0, 0)),
            ),
        )?;
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "yuzu_soft_shocked",
    yuzu_soft_shocked,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["震惊柚子厨"],
    tags = MemeTags::yuzu_soft(),
    date_created = local_date(2024, 7, 26),
    date_modified = local_date(2026, 4, 12),
);
