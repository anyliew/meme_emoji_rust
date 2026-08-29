use rand::RngExt;
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

use crate::{options::number_option, register_meme};

number_option!(Number, 1, 4);

fn national_day_plan(
    images: Vec<InputImage>,
    texts: Vec<String>,
    options: Number,
) -> Result<Vec<u8>, Error> {
    let name = if !texts.is_empty() {
        texts[0].clone()
    } else if !images[0].name.is_empty() {
        images[0].name.clone()
    } else {
        "他".to_string()
    };
    let text = format!("{name}の国庆计划");
    let num = options.number.unwrap_or(rand::rng().random_range(1..=4));
    let configs = [
        ((152, 28), (210, 210), IRect::from_ltrb(412, 28, 1175, 238)),
        ((18, 12), (125, 125), IRect::from_ltrb(152, 12, 694, 137)),
        ((54, 25), (220, 220), IRect::from_ltrb(300, 25, 1000, 240)),
        ((38, 16), (118, 118), IRect::from_ltrb(168, 16, 697, 133)),
    ];
    let ((ax, ay), (aw, ah), rect) = configs[num as usize - 1];
    let frame = load_image(format!("national_day_plan/{}.png", num - 1))?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let img = images[0].circle().resize_exact((aw, ah));
        canvas.draw_image(&img, (ax, ay), None);
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_text_area_auto_font_size(
            rect,
            &text,
            15.0,
            120.0,
            text_params!(
                font_families = &["FZXS14"],
                text_align = TextAlign::Left,
                paint = new_paint(Color::from_rgb(0, 0, 0)),
            ),
        )?;
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "national_day_plan",
    national_day_plan,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["国庆计划", "国庆节计划"],
    date_created = local_date(2025, 9, 28),
    date_modified = local_date(2025, 9, 29),
);
