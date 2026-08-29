use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn lemon(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let image = images[0].image.circle().resize_exact((90, 90));
    let frames = (1..=92)
        .map(|i| load_image(format!("lemon/{i}.png")))
        .collect::<Result<Vec<_>, _>>()?;
    let frame_size = frames.iter().fold((0, 0), |(w, h), frame| {
        (w.max(frame.width()), h.max(frame.height()))
    });
    let mut encoder = GifEncoder::new();
    for frame in &frames {
        let mut surface = new_surface(frame_size);
        let canvas = surface.canvas();
        canvas.clear(Color::TRANSPARENT);
        canvas.draw_image(&image, (18, 15), None);
        canvas.draw_image(frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.06)?;
    }
    encoder.finish()
}

register_meme!(
    "lemon",
    lemon,
    min_images = 1,
    max_images = 1,
    keywords = &["柠檬"],
    date_created = local_date(2025, 7, 11),
    date_modified = local_date(2025, 7, 11),
);
