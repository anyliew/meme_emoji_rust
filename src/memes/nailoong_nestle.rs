use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn nailoong_nestle(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [(160, 9)];
    let sizes = [(120, 120)];
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("nailoong_nestle/{i}.png"))?;
        let user_head = images[0].resize_exact(sizes[i]).circle();
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_image(&user_head, positions[i], None);
        Ok(surface.image_snapshot())
    };
    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 1,
            duration: 0.13,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "nailoong_nestle",
    nailoong_nestle,
    min_images = 1,
    max_images = 1,
    keywords = &["奶龙依偎"],
    date_created = local_date(2026, 10, 10),
    date_modified = local_date(2026, 10, 10),
);
