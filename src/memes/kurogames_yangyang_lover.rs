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

use crate::tags::MemeTags;
use crate::{options::NoOptions, register_meme};

fn kurogames_yangyang_lover(
    images: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let name = if !texts.is_empty() {
        texts[0].clone()
    } else if !images[0].name.is_empty() {
        images[0].name.clone()
    } else {
        "漂泊旅人".to_string()
    };
    let text = format!("这是我念念不忘的漂泊者{name},是秧秧的老公~");
    let frame = load_image("kurogames_yangyang_lover/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(0, 1443, 1440, 1761),
        &text,
        20.0,
        100.0,
        text_params!(
            font_families = &["FZShaoEr-M11S"],
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        let image = images[0].resize_exact((600, 600));
        canvas.draw_image(&image, (426, 845), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "kurogames_yangyang_lover",
    kurogames_yangyang_lover,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["秧秧老公"],
    tags = MemeTags::wuthering_waves(),
    date_created = local_date(2025, 6, 10),
    date_modified = local_date(2025, 6, 10),
);
