use skia_safe::{Color, IRect, Image, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint},
};

use crate::{options::NoOptions, register_meme};

const AVATAR_SLOTS: [(i32, i32, i32, i32); 15] = [
    (77, 710, 220, 220),
    (77, 968, 220, 220),
    (322, 429, 246, 246),
    (599, 429, 246, 246),
    (350, 710, 485, 485),
    (350, 1255, 215, 215),
    (620, 1255, 495, 495),
    (891, 710, 220, 220),
    (891, 968, 220, 220),
    (891, 1793, 220, 220),
    (1163, 429, 500, 500),
    (1163, 968, 500, 500),
    (1163, 1508, 247, 247),
    (1709, 688, 244, 244),
    (1709, 974, 244, 244),
];

fn heart_shape(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = texts.first().cloned();
    let frame = load_image("heart_shape/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        for (i, &(x, y, w, h)) in AVATAR_SLOTS.iter().enumerate() {
            let img = images[i % images.len()].square().resize_exact((w, h));
            canvas.draw_image(&img, (x, y), None);
        }
        if let Some(text) = &text {
            canvas.draw_text_area_auto_font_size(
                IRect::from_ltrb(180, 93, 1868, 295),
                text.clone(),
                15.0,
                300.0,
                text_params!(
                    font_families = &["FZKaTong-M19S"],
                    text_align = TextAlign::Center,
                    paint = new_paint(Color::WHITE),
                ),
            )?;
        }
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "heart_shape",
    heart_shape,
    min_images = 1,
    max_images = 15,
    min_texts = 0,
    max_texts = 1,
    keywords = &["爱心形状"],
    date_created = local_date(2026, 10, 12),
    date_modified = local_date(2026, 10, 12),
);
