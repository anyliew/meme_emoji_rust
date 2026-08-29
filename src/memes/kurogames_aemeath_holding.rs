use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn kurogames_aemeath_holding(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("kurogames_aemeath_holding/0.png")?;
    let mut encoder = GifEncoder::new();
    for i in 0..36 {
        let img = images[0]
            .image
            .rotate(-(i as f32 * 10.0))
            .resize_exact((91, 91));
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        canvas.draw_image(&img, (43, 159), None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.05)?;
    }
    encoder.finish()
}

register_meme!(
    "kurogames_aemeath_holding",
    kurogames_aemeath_holding,
    min_images = 1,
    max_images = 1,
    keywords = &["爱弥斯捧"],
    tags = MemeTags::wuthering_waves(),
    date_created = local_date(2026, 2, 24),
    date_modified = local_date(2026, 2, 24),
);
