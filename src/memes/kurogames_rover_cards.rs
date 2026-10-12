use skia_safe::{Color, IRect, Image, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    text_params,
    tools::{load_image, local_date, new_paint},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn kurogames_rover_cards(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let name = texts.first().unwrap_or(&images[0].name);
    let text = format!("{name} 出来战斗吧");
    let frame = load_image("kurogames_rover_cards/0.png")?;

    let func = |images: Vec<Image>| {
        // 创建新的surface，先绘制用户图片作为最底层
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        
        // 先绘制用户图片作为最底层
        let img = images[0]
            .resize_bound((80, 80), Fit::Cover)
            .rotate(-30.0);
        canvas.draw_image(&img, (417, 247), None);
        
        // 然后绘制frame作为中层（覆盖在用户图片之上）
        canvas.draw_image(&frame, (0, 0), None);
        
        // 最后绘制文字作为最上层
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(1, 1, 960, 185),
            &text,
            20.0,
            120.0,
            text_params!(
                font_families = &["FZShaoEr-M11S"],
                text_align = TextAlign::Center,
                paint = new_paint(Color::from_rgb(255, 255, 255))
            ),
        )?;
        
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "kurogames_rover_cards",
    kurogames_rover_cards,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    tags = MemeTags::wuthering_waves(),
    keywords = &["荣耀之丘"],
    date_created = local_date(2025, 7, 7),
    date_modified = local_date(2026, 10, 12),
);