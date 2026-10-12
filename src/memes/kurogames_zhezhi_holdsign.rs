use skia_safe::{ClipOp, Color, IRect, Matrix, Rect, textlayout::TextAlign};

use crate::{options::NoOptions, register_meme, tags::MemeTags};
use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{default_sampling_options, load_image, local_date, new_paint, new_surface},
};

const DEFAULT_TEXT: &str = "祝你鸣潮玩的开心";

fn kurogames_zhezhi_holdsign(
    _: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { DEFAULT_TEXT };
    let frame = load_image("kurogames_zhezhi_holdsign/0.png")?;

    let mut text_surface = new_surface((550, 400));
    let text_canvas = text_surface.canvas();
    text_canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(5, 5, 540, 390),
        text,
        30.0,
        120.0,
        text_params!(
            font_families = &["FZSJ-QINGCRJ"],
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;
    let text_img = text_surface.image_snapshot();

    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.save();
    canvas.translate((306, 417));
    canvas.clip_rect(Rect::from_isize((828, 666)), ClipOp::Intersect, false);
    canvas.concat(&Matrix::new_all(
        1.4322355996,
        0.0667393379,
        0.0,
        -0.1392956674,
        1.3333332585,
        100.0,
        0.0001146597,
        -0.0001986435,
        1.0,
    ));
    canvas.draw_image_with_sampling_options(&text_img, (0, 0), default_sampling_options(), None);
    canvas.restore();

    encode_png(surface.image_snapshot())
}

register_meme!(
    "kurogames_zhezhi_holdsign",
    kurogames_zhezhi_holdsign,
    min_texts = 1,
    max_texts = 1,
    tags = MemeTags::wuthering_waves(),
    default_texts = &[DEFAULT_TEXT],
    keywords = &["折枝举牌"],
    date_created = local_date(2025, 5, 17),
    date_modified = local_date(2025, 5, 17),
);
