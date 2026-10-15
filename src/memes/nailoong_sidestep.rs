use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn nailoong_sidestep(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (121, 13), (121, 13), (122, 12), (120, 12), (120, 12),
        (120, 12), (120, 12), (120, 12), (121, 12), (124, 13),
        (129, 25), (128, 26), (123, 30), (97, 24), (92, 21),
        (81, 18), (75, 17), (72, 15), (67, 13), (65, 13),
        (62, 13), (60, 13), (60, 13), (59, 13), (58, 14),
        (58, 14), (58, 14), (58, 14), (58, 14), (58, 14),
        (58, 12), (51, 15), (48, 23), (49, 28), (57, 31),
        (75, 27), (94, 23), (102, 21), (113, 17), (115, 17),
        (120, 15), (120, 15), (120, 15),
    ];
    let sizes = [(63, 63); 43];
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("nailoong_sidestep/{i}.png"))?;
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
            frame_num: 43,
            duration: 0.07,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "nailoong_sidestep",
    nailoong_sidestep,
    min_images = 1,
    max_images = 1,
    keywords = &["闪身步", "奶龙闪身步"],
    date_created = local_date(2026, 10, 14),
    date_modified = local_date(2026, 10, 14),
);
