use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn nailoong_snuggle(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (112, 1), (112, 1), (112, 1), (112, 1), (112, 0),
        (111, 0), (106, 0), (104, 0), (97, 0), (92, 0),
        (89, 0), (83, 0), (80, 0), (75, 0), (73, 0),
        (72, 0), (70, 0), (70, -1), (72, 0), (75, 0),
        (80, 0), (85, 0), (95, 0), (99, 0), (106, 0),
        (113, 1), (117, 1), (122, 3), (123, 3), (124, 3),
        (123, 4), (123, 4), (115, 4), (113, 2), (107, 1),
        (99, 0), (97, -1), (86, -2), (81, -2), (77, -2),
        (72, 0), (70, 1), (70, 1), (70, 1), (70, 1),
        (73, 0), (77, 0), (83, -1), (91, -2), (97, -1),
        (108, 0), (115, 0), (116, 0), (123, 2), (126, 3),
        (128, 4), (128, 4), (126, 5), (117, 3), (114, 2),
        (110, 0), (99, 0), (96, -1), (88, -1), (81, -1),
        (79, -1), (72, -1), (70, 0), (68, 1), (66, 2),
        (66, 2), (66, 2), (69, 2), (73, 0), (77, 0),
    ];
    let sizes = [
        (91, 91), (91, 91), (91, 91), (91, 91), (91, 91),
        (91, 91), (91, 91), (91, 91), (91, 91), (91, 91),
        (91, 91), (91, 91), (91, 91), (91, 91), (91, 91),
        (91, 91), (93, 93), (93, 93), (91, 91), (91, 91),
        (91, 91), (91, 91), (91, 91), (91, 91), (91, 91),
        (91, 91), (91, 91), (91, 91), (91, 91), (91, 91),
        (91, 91), (91, 91), (91, 91), (91, 91), (91, 91),
        (91, 91), (91, 91), (91, 90), (91, 90), (91, 91),
        (91, 91), (91, 91), (91, 91), (91, 91), (91, 91),
        (91, 91), (91, 91), (91, 91), (91, 91), (91, 91),
        (91, 91), (91, 91), (91, 91), (91, 91), (91, 91),
        (91, 91), (91, 91), (91, 91), (93, 93), (91, 91),
        (91, 91), (91, 91), (91, 91), (91, 91), (91, 91),
        (91, 91), (91, 91), (91, 91), (91, 91), (91, 91),
        (91, 91), (91, 91), (91, 91), (91, 91), (91, 91),
    ];
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("nailoong_snuggle/{i}.png"))?;
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
            frame_num: 75,
            duration: 0.13,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "nailoong_snuggle",
    nailoong_snuggle,
    min_images = 1,
    max_images = 1,
    keywords = &["奶龙贴贴"],
    date_created = local_date(2026, 10, 10),
    date_modified = local_date(2026, 10, 10),
);
