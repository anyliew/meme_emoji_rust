use skia_safe::{Color, IRect, Image, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::register_meme;

#[derive(MemeOptions)]
pub(crate) struct NewGoodnewsOptions {
    #[option(short, long, description = "昵称（第一段文字）")]
    name: Option<String>,
}

fn new_goodnews(
    images: Vec<InputImage>,
    texts: Vec<String>,
    options: NewGoodnewsOptions,
) -> Result<Vec<u8>, Error> {
    let name = options
        .name
        .clone()
        .filter(|n| !n.is_empty())
        .or_else(|| texts.first().cloned())
        .unwrap_or_else(|| "天命之人".to_string());
    let text2 = texts.first().cloned().unwrap_or_default();
    let frame = load_image("new_goodnews/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(363, 910, 719, 985),
        &name,
        10.0,
        100.0,
        text_params!(
            font_families = &["FZShaoEr-M11S"],
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(28, 28, 28)),
        ),
    )?;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(297, 1055, 775, 1379),
        &text2,
        10.0,
        60.0,
        text_params!(
            font_families = &["FZXS14"],
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(249, 250, 161)),
        ),
    )?;
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        let img = images[0].resize_exact((445, 445));
        canvas.draw_image(&img, (319, 408), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "new_goodnews",
    new_goodnews,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["喜报传佳讯\n福星高照\n满门庭"],
    keywords = &["新喜报"],
    date_created = local_date(2024, 7, 26),
    date_modified = local_date(2026, 4, 11),
);
