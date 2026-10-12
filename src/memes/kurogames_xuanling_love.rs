use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn kurogames_xuanling_love(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let positions = [
        (62, 197), (63, 197), (62, 196), (62, 196), (62, 195),
        (62, 195), (62, 195), (62, 195), (62, 195), (62, 195),
        (61, 196), (60, 194), (58, 193), (56, 193), (55, 192),
        (54, 191), (53, 191), (53, 190), (51, 190), (51, 190),
        (52, 190), (51, 191), (50, 189), (50, 189), (50, 189),
        (50, 189), (50, 189), (50, 189), (50, 189), (50, 189),
        (50, 189), (50, 189), (50, 189), (50, 189), (50, 189),
        (50, 189), (50, 189), (50, 189), (50, 189), (51, 189),
        (50, 191), (50, 190), (50, 190), (50, 190), (50, 190),
        (50, 191), (50, 190), (51, 191), (50, 191), (50, 191),
        (50, 191), (50, 191), (51, 191), (50, 191), (50, 191),
        (50, 192), (50, 192), (50, 192), (50, 192), (52, 192),
    ];
    let sizes = [
        (110, 79), (109, 79), (110, 80), (109, 80), (109, 81),
        (110, 80), (110, 81), (109, 80), (110, 81), (110, 81),
        (110, 80), (111, 82), (113, 83), (114, 83), (115, 85),
        (116, 85), (116, 85), (116, 87), (118, 86), (118, 86),
        (116, 87), (118, 86), (118, 87), (119, 87), (119, 88),
        (119, 87), (118, 87), (118, 87), (119, 87), (118, 88),
        (119, 87), (119, 87), (118, 88), (118, 87), (119, 87),
        (119, 87), (119, 87), (119, 87), (118, 87), (118, 87),
        (118, 86), (118, 87), (119, 86), (119, 87), (118, 88),
        (119, 86), (119, 87), (118, 86), (119, 87), (118, 87),
        (119, 87), (118, 86), (117, 87), (118, 87), (119, 88),
        (118, 86), (118, 87), (119, 86), (119, 87), (117, 86),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("kurogames_xuanling_love/{i}.png"))?;
        let head = images[0].square().resize_fit(sizes[i], Fit::Cover);
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&head, positions[i], None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 60,
            duration: 0.03,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "kurogames_xuanling_love",
    kurogames_xuanling_love,
    min_images = 1,
    max_images = 1,
    keywords = &["玄翎爱心"],
    tags = MemeTags::wuthering_waves(),
    date_created = local_date(2026, 9, 11),
    date_modified = local_date(2026, 9, 11),
);
