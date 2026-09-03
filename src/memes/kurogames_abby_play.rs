use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn kurogames_abby_play(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let locs = [
        (794, 402), (787, 402), (786, 402), (787, 423), (789, 354),
        (787, 262), (791, 172), (793, 128), (793, 131), (792, 140),
        (793, 154), (793, 174), (794, 198), (793, 223), (792, 250),
        (790, 276), (793, 301), (793, 330), (793, 352), (793, 373),
        (793, 390), (796, 399), (796, 402), (794, 403), (790, 422),
        (794, 354), (797, 264), (797, 174), (796, 124), (796, 127),
        (793, 138), (793, 157), (793, 180), (793, 204), (793, 232),
        (793, 265), (793, 291), (793, 322), (793, 348), (793, 368),
        (793, 387), (792, 399), (792, 402), (792, 402), (792, 402),
        (793, 402), (793, 402),
    ];
    let image = images[0].image.square().resize_exact((177, 177));
    let mut encoder = GifEncoder::new();
    for (i, (x, y)) in locs.iter().enumerate() {
        let frame = load_image(format!("kurogames_abby_play/{}.png", i + 1))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::TRANSPARENT);
        canvas.draw_image(&image, (*x, *y), None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.03)?;
    }
    encoder.finish()
}

register_meme!(
    "kurogames_abby_play",
    kurogames_abby_play,
    min_images = 1,
    max_images = 1,
    keywords = &["阿布顶", "阿布玩"],
    tags = MemeTags::wuthering_waves(),
    date_created = local_date(2026, 2, 16),
    date_modified = local_date(2026, 2, 16),
);
