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

use crate::{options::NoOptions, register_meme};

fn azur_lane_cheshire_thumbs_up(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let name = texts.first().unwrap_or(&images[0].name);
    let text = format!("{name},你真是一个大聪明\n柴郡为你点个赞👍🏻");
    let frame = load_image("azur_lane_cheshire_thumbs_up/0.jpg")?;

    let func = |images: Vec<Image>| {
        // 创建新的surface，先绘制frame作为最底层
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        
        // 先绘制frame作为最底层
        canvas.draw_image(&frame, (0, 0), None);
        
        // 然后绘制文字作为中层
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(207, 1, 739, 150),
            &text,
            20.0,
            100.0,
            text_params!(
                font_families = &["FZXS14"],
                text_align = TextAlign::Left,
                paint = new_paint(Color::from_rgb(255, 255, 255))
            ),
        )?;
        
        // 最后绘制用户图片作为最上层
        let img = images[0].circle().resize_bound((230, 230), Fit::Cover);
        canvas.draw_image(&img, (30, 120), None);
        
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "azur_lane_cheshire_thumbs_up",
    azur_lane_cheshire_thumbs_up,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["柴郡点赞","柴郡猫点赞"],
    date_created = local_date(2025, 7, 2),
    date_modified = local_date(2025, 7, 2),
);