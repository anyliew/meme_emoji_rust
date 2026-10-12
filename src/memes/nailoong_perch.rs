use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn nailoong_perch(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (98, 52), (93, 58), (64, 57), (61, 55), (53, 46),
        (45, 38), (46, 39), (50, 40), (62, 40), (74, 36),
        (95, 31), (99, 22), (107, 21), (113, 23), (109, 24),
        (98, 29), (86, 31), (61, 32), (56, 32), (52, 34),
        (50, 33), (52, 32), (57, 32), (76, 33), (88, 27),
        (108, 20), (127, 24), (136, 26), (155, 38), (148, 40),
        (132, 46), (114, 54), (101, 55), (75, 58), (68, 60),
        (62, 59), (62, 55), (62, 55),
    ];
    let sizes = [
        (63, 63), (63, 63), (63, 63), (63, 63), (63, 63),
        (63, 63), (63, 63), (63, 63), (63, 63), (63, 63),
        (63, 63), (63, 63), (63, 63), (63, 63), (63, 63),
        (63, 63), (63, 63), (63, 63), (63, 63), (63, 63),
        (63, 63), (63, 63), (63, 63), (63, 63), (65, 65),
        (63, 63), (63, 63), (63, 63), (63, 63), (63, 63),
        (63, 63), (63, 63), (63, 63), (63, 63), (63, 63),
        (63, 63), (63, 63), (63, 63),
    ];
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("nailoong_perch/{i}.png"))?;
        let user_head = images[0].resize_exact(sizes[i]);
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&user_head, positions[i], None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };
    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 38,
            duration: 0.07,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "nailoong_perch",
    nailoong_perch,
    min_images = 1,
    max_images = 1,
    keywords = &["趴墩"],
    date_created = local_date(2026, 10, 12),
    date_modified = local_date(2026, 10, 12),
);
