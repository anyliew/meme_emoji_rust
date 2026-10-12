use skia_safe::{Color, IRect, Image, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn kurogames_verina_finger(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("kurogames_verina_finger/0.png")?;
    
    let name = texts.first().unwrap_or(&images[0].name);
    
    let text = format!("维里奈:坏消息,{name}抽卡又歪了\n维里奈:好消息,歪的是我的共鸣链😆😁\n维里奈:哈哈哈😆😁");

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        
        let img = images[0].resize_exact((420, 420));
        canvas.draw_image(&img, (248, 555), None);

        canvas.draw_image(&frame, (0, 0), None);
        
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(1, 1, 1203, 257),
            &text,
            20.0,
            100.0,
            text_params!(
                font_families = &["FZXS14"],
                text_align = TextAlign::Left,
                paint = new_paint(Color::from_rgb(0, 0, 0))
            ),
        )?;
        
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "kurogames_verina_finger",
    kurogames_verina_finger,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    tags = MemeTags::wuthering_waves(),
    keywords = &["维里奈指", "小维指"],
    date_created = local_date(2025, 7, 7),
    date_modified = local_date(2026, 10, 12),
);