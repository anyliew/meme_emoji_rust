use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

const AVATAR_SLOTS: [(i32, i32, i32, i32); 15] = [
    (323, 73, 242, 241),
    (599, 73, 242, 241),
    (1163, 73, 501, 500),
    (1710, 332, 242, 241),
    (77, 355, 218, 218),
    (350, 355, 482, 482),
    (891, 355, 218, 218),
    (77, 613, 218, 218),
    (1163, 613, 501, 499),
    (891, 619, 218, 217),
    (1710, 619, 247, 247),
    (352, 899, 213, 213),
    (620, 899, 489, 491),
    (1163, 1153, 242, 241),
    (891, 1433, 218, 220),
];

fn heart_shape(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("heart_shape/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        for (i, &(x, y, w, h)) in AVATAR_SLOTS.iter().enumerate() {
            let img = images[i % images.len()].square().resize_exact((w, h));
            canvas.draw_image(&img, (x, y), None);
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
    max_texts = 0,
    keywords = &["爱心形状"],
    date_created = local_date(2026, 10, 12),
    date_modified = local_date(2026, 10, 16),
);
