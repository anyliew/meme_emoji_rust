use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn kurogames_suisui_love(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let positions = [
        (0, 68), (0, 73), (0, 77), (0, 81), (0, 86),
        (0, 92), (0, 97), (0, 100), (0, 92), (0, 82),
        (0, 71), (0, 60), (0, 48), (0, 36), (0, 25),
        (0, 20), (0, 21), (0, 21), (0, 22), (0, 24),
        (0, 26), (0, 28), (0, 31), (0, 35), (0, 40),
        (0, 46), (0, 51), (0, 57), (0, 63), (0, 68),
    ];
    let sizes = [
        (143, 102), (144, 97), (145, 93), (147, 89), (150, 84),
        (153, 78), (157, 73), (161, 70), (162, 78), (164, 88),
        (165, 99), (166, 110), (166, 122), (166, 134), (166, 140),
        (167, 146), (167, 149), (166, 149), (165, 148), (163, 146),
        (160, 144), (157, 142), (155, 139), (152, 135), (149, 130),
        (147, 124), (145, 119), (144, 113), (143, 107), (143, 102),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("kurogames_suisui_love/{i}.png"))?;
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
            frame_num: 30,
            duration: 0.06,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "kurogames_suisui_love",
    kurogames_suisui_love,
    min_images = 1,
    max_images = 1,
    keywords = &["穗穗爱心"],
    tags = MemeTags::wuthering_waves(),
    date_created = local_date(2026, 9, 11),
    date_modified = local_date(2026, 9, 11),
);
