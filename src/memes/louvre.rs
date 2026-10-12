use rand::RngExt;
use skia_safe::{AlphaType, image::CachingHint, Color, ColorType, Data, Image, ImageInfo, Rect, images};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    encoder::make_png_or_gif,
    image::ImageExt,
    text::Text2Image,
    text_params,
    tools::{load_image, local_date, new_paint, new_stroke_paint},
};

use crate::register_meme;

#[derive(MemeOptions)]
struct LouvreOptions {
    /// 线迹模式: 0=随机选择, 1=精细, 2=一般, 3=稍粗, 4=超粗, 5=极粗, 6=浮雕, 7=彩色线稿
    #[option(short, long, default = 0)]
    number: Option<i32>,
    /// Kiss 彩色渐变: 0=关(铅笔灰度), 1=开
    #[option(long, default = 1)]
    color: Option<i32>,
    /// 调子阴影: 0=关, 1=开
    #[option(long, default = 1)]
    shade: Option<i32>,
    /// 降噪: 0=关, 1=开
    #[option(long, default = 1)]
    denoise: Option<i32>,
    /// 方形裁剪(专辑封面比例): 0=关, 1=开
    #[option(long, default = 0)]
    cover: Option<i32>,
    /// 线迹轻重 80~126
    #[option(long, default = 118, minimum = 80, maximum = 126)]
    dark_cut: Option<i32>,
    /// 调子阈值 20~200
    #[option(long, default = 120, minimum = 20, maximum = 200)]
    shade_limit: Option<i32>,
    /// 亮度 -100~100
    #[option(long, default = 0, minimum = -100, maximum = 100)]
    light: Option<i32>,
    /// 铅笔纹理强度 0~1
    #[option(long, default = 0.4, minimum = 0.0, maximum = 1.0)]
    texture: Option<f32>,
    /// 叠印歌词+专辑封面: 0=关, 1=开
    #[option(long, default = 0)]
    lyric: Option<i32>,
}

const SHADE_LIGHT: f32 = 108.0;
const LIGHT_CUT: f32 = 128.0;

const STOPS_COLOR: [(f32, (f32, f32, f32)); 6] = [
    (0.0, (251.0 / 255.0, 186.0 / 255.0, 48.0 / 255.0)),
    (0.4, (252.0 / 255.0, 114.0 / 255.0, 53.0 / 255.0)),
    (0.6, (252.0 / 255.0, 53.0 / 255.0, 78.0 / 255.0)),
    (0.7, (207.0 / 255.0, 54.0 / 255.0, 223.0 / 255.0)),
    (0.8, (55.0 / 255.0, 181.0 / 255.0, 217.0 / 255.0)),
    (1.0, (62.0 / 255.0, 182.0 / 255.0, 218.0 / 255.0)),
];

const STOPS_GRAY: [(f32, (f32, f32, f32)); 2] = [
    (0.0, (245.0 / 255.0, 245.0 / 255.0, 245.0 / 255.0)),
    (1.0, (208.0 / 255.0, 208.0 / 255.0, 208.0 / 255.0)),
];

fn read_luma_plane(image: &Image) -> Result<(Vec<u8>, Vec<u8>), Error> {
    let info = ImageInfo::new(
        image.dimensions(),
        ColorType::RGBA8888,
        AlphaType::Unpremul,
        None,
    );
    let row_bytes = info.min_row_bytes();
    let mut data = vec![0u8; info.compute_min_byte_size()];
    if !image.read_pixels(&info, &mut data, row_bytes, (0, 0), CachingHint::Allow) {
        return Err(Error::ImageDecodeError("read pixels failed".to_string()));
    }
    let count = (image.width() * image.height()) as usize;
    let mut luma = vec![0u8; count];
    let mut alpha = vec![0u8; count];
    for i in 0..count {
        let p = i * 4;
        luma[i] = (0.299 * data[p] as f32 + 0.587 * data[p + 1] as f32 + 0.114 * data[p + 2] as f32)
            as u8;
        alpha[i] = data[p + 3];
    }
    Ok((luma, alpha))
}

fn make_kernel(mode: i32) -> (usize, Vec<f32>) {
    match mode {
        6 => (
            3,
            vec![1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0],
        ),
        _ => {
            let size = match mode {
                1 => 5,
                2 => 7,
                3 => 9,
                4 => 11,
                _ => 13,
            };
            let value = 1.0 / (size * size) as f32;
            (size, vec![value; size * size])
        }
    }
}

fn convolve(src: &[u8], width: usize, height: usize, kernel_size: usize, kernel: &[f32]) -> Vec<u8> {
    let radius = kernel_size as i32 / 2;
    let mut out = vec![0u8; width * height];
    for y in 0..height {
        for x in 0..width {
            let mut acc = 0.0f32;
            for ky in 0..kernel_size {
                for kx in 0..kernel_size {
                    let sx = (x as i32 + kx as i32 - radius).clamp(0, width as i32 - 1) as usize;
                    let sy = (y as i32 + ky as i32 - radius).clamp(0, height as i32 - 1) as usize;
                    acc += src[sy * width + sx] as f32 * kernel[ky * kernel_size + kx];
                }
            }
            out[y * width + x] = acc.clamp(0.0, 255.0) as u8;
        }
    }
    out
}

fn box_blur(src: &[u8], width: usize, height: usize, radius: i32) -> Vec<u8> {
    let mut out = vec![0u8; width * height];
    for y in 0..height {
        for x in 0..width {
            let mut sum = 0u32;
            let mut count = 0u32;
            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    let sx = (x as i32 + dx).clamp(0, width as i32 - 1) as usize;
                    let sy = (y as i32 + dy).clamp(0, height as i32 - 1) as usize;
                    sum += src[sy * width + sx] as u32;
                    count += 1;
                }
            }
            out[y * width + x] = (sum / count) as u8;
        }
    }
    out
}

fn gradient(width: usize, height: usize, color: bool) -> Vec<(f32, f32, f32)> {
    let stops: &[(f32, (f32, f32, f32))] = if color { &STOPS_COLOR } else { &STOPS_GRAY };
    let w = width as f32;
    let h = height as f32;
    let denom = w * w + h * h;
    let mut out = Vec::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            let t = ((x as f32 * w + y as f32 * h) / denom).clamp(0.0, 1.0);
            let mut i = 0;
            while i < stops.len() - 2 && t > stops[i + 1].0 {
                i += 1;
            }
            let (t0, c0) = stops[i];
            let (t1, c1) = stops[i + 1];
            let f = if t1 > t0 { (t - t0) / (t1 - t0) } else { 0.0 };
            out.push((
                (c0.0 + (c1.0 - c0.0) * f).clamp(0.0, 1.0),
                (c0.1 + (c1.1 - c0.1) * f).clamp(0.0, 1.0),
                (c0.2 + (c1.2 - c0.2) * f).clamp(0.0, 1.0),
            ));
        }
    }
    out
}

fn parse_lyrics() -> Vec<(String, Option<String>)> {
    let path = meme_generator_utils::config::IMAGES_DIR.join("louvre/one-last-kiss.lrc");
    let Ok(text) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    let mut lyrics = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if !line.starts_with('[') {
            continue;
        }
        let Some(end) = line.find(']') else {
            continue;
        };
        let time_part = &line[1..end];
        let Some(sep) = time_part.find(':') else {
            continue;
        };
        let Ok(_minutes) = time_part[..sep].trim().parse::<i64>() else {
            continue;
        };
        if time_part[sep + 1..].parse::<f64>().is_err() {
            continue;
        }
        let rest = line[end + 1..].trim();
        let (jp, cn) = match rest.find('「') {
            Some(idx) if rest.ends_with('」') => (
                rest[..idx].trim().to_string(),
                Some(rest[idx + '「'.len_utf8()..rest.len() - '」'.len_utf8()].trim().to_string()),
            ),
            _ => (rest.to_string(), None),
        };
        if !jp.is_empty() {
            lyrics.push((jp, cn));
        }
    }
    lyrics
}

fn overlay_lyric(
    canvas: &skia_safe::Canvas,
    width: i32,
    height: i32,
    cover: &Image,
    jp: &str,
    cn: Option<&str>,
) {
    let margin = ((height as f32 * 0.02) as i32).max(10);
    let jp_size = ((height as f32 * 0.045) as i32).max(16);
    let cn_size = ((height as f32 * 0.03) as i32).max(12);
    let stroke = |size: i32| new_stroke_paint(Color::from_rgb(15, 15, 15), (size as f32 * 0.06).max(1.0));

    let jp_img = Text2Image::from_text(
        jp,
        jp_size as f32,
        text_params!(
            paint = new_paint(Color::from_rgb(255, 255, 255)),
            stroke_paint = stroke(jp_size),
        ),
    );
    let cn_img = cn.map(|t| {
        Text2Image::from_text(
            t,
            cn_size as f32,
            text_params!(
                paint = new_paint(Color::from_rgb(235, 235, 235)),
                stroke_paint = stroke(cn_size),
            ),
        )
    });

    let jp_h = jp_img.height().ceil() as i32;
    let cn_h = cn_img.as_ref().map(|t| t.height().ceil() as i32).unwrap_or(0);
    let y = height - margin - jp_h - cn_h;
    jp_img.draw_on_canvas(canvas, (margin as f32, y as f32));
    if let Some(cn_img) = &cn_img {
        cn_img.draw_on_canvas(canvas, (margin as f32, (y + jp_h) as f32));
    }

    let sz = ((height as f32 * 0.14) as i32).max(40);
    let border = ((sz as f32 * 0.04) as i32).max(2);
    let cx = width - sz - margin;
    let cy = height - sz - margin;
    let mut paint = new_paint(Color::from_rgb(255, 255, 255));
    paint.set_anti_alias(false);
    canvas.draw_rect(
        Rect::from_xywh(
            (cx - border) as f32,
            (cy - border) as f32,
            (sz + 2 * border) as f32,
            (sz + 2 * border) as f32,
        ),
        &paint,
    );
    let cover_resized = cover.resize_exact((sz, sz));
    canvas.draw_image(&cover_resized, (cx, cy), None);
}

fn louvre(
    images: Vec<InputImage>,
    _: Vec<String>,
    options: LouvreOptions,
) -> Result<Vec<u8>, Error> {
    let number = options.number.unwrap_or(0);
    let mode = if number == 0 {
        rand::rng().random_range(1..=7)
    } else if (1..=7).contains(&number) {
        number
    } else {
        return Err(Error::MemeFeedback(
            "模式编号错误，请选择 1~7 或 0（随机）".to_string(),
        ));
    };
    let (kernel_size, kernel) = make_kernel(mode);

    let color_on = options.color.unwrap_or(1) > 0;
    let shade_on = options.shade.unwrap_or(1) > 0;
    let denoise_on = options.denoise.unwrap_or(1) > 0;
    let cover_on = options.cover.unwrap_or(0) > 0;
    let lyric_on = options.lyric.unwrap_or(0) > 0;
    let dark_cut = options.dark_cut.unwrap_or(118).clamp(80, 126) as f32;
    let shade_limit = options.shade_limit.unwrap_or(120).clamp(20, 200) as u8;
    let light = options.light.unwrap_or(0).clamp(-100, 100);
    let texture = options.texture.unwrap_or(0.4).clamp(0.0, 1.0);

    let pencil_image = load_image("louvre/pencil-texture.jpg")?;
    let cover_image = load_image("louvre/One_Last_Kiss.JPG")?;
    let lyrics = parse_lyrics();

    let func = |images: Vec<Image>| {
        let img = if cover_on { images[0].square() } else { images[0].clone() };
        let width = img.width() as usize;
        let height = img.height() as usize;
        let (mut l, a) = read_luma_plane(&img)?;

        if light != 0 {
            let factor = 1.0 + light as f32 / 100.0;
            for v in l.iter_mut() {
                *v = ((*v as f32) * factor).clamp(0.0, 255.0) as u8;
            }
        }

        let mut plane = vec![0u8; width * height];
        for i in 0..(width * height) {
            let lf = l[i] as f32;
            let af = a[i] as f32 / 255.0;
            plane[i] = (lf * af + 255.0 * (1.0 - af)) as u8;
        }

        let pencil = pencil_image.resize_exact((width as i32, height as i32));
        let (pencil_gray, _) = read_luma_plane(&pencil)?;

        let mut shade = vec![0u8; width * height];
        if shade_on {
            for i in 0..(width * height) {
                shade[i] = if plane[i] > shade_limit { 0 } else { 255 };
            }
            shade = box_blur(&shade, width, height, 3);
            for i in 0..(width * height) {
                let v = shade[i] as f32 * (255.0 - pencil_gray[i] as f32) / 255.0;
                shade[i] = (v * SHADE_LIGHT / 255.0) as u8;
            }
        }

        let plane = if denoise_on {
            let denoise_kernel = vec![1.0f32 / 9.0; 9];
            convolve(&plane, width, height, 3, &denoise_kernel)
        } else {
            plane
        };
        let blurred = convolve(&plane, width, height, kernel_size, &kernel);

        let scale = (255.0 - LIGHT_CUT - dark_cut) / 255.0;
        let mut mask = vec![0u8; width * height];
        for i in 0..(width * height) {
            let diff = (plane[i] as f32 - blurred[i] as f32 + 128.0).clamp(0.0, 255.0);
            let value = (((diff - dark_cut) / scale).round()).clamp(0.0, 255.0);
            mask[i] = (255.0 - value).max(shade[i] as f32) as u8;
        }

        let grad = gradient(width, height, color_on);
        let mut out = vec![0u8; width * height * 4];
        for i in 0..(width * height) {
            let m = mask[i] as f32 / 255.0;
            let (gr, gg, gb) = grad[i];
            let mut r = gr * m + (1.0 - m);
            let mut g = gg * m + (1.0 - m);
            let mut b = gb * m + (1.0 - m);

            if texture > 0.0 && pencil_gray[i] > 100 {
                r = (r * 255.0 - 20.0 * texture) / 255.0;
                g = (g * 255.0 - 20.0 * texture) / 255.0;
                b = (b * 255.0 - 20.0 * texture) / 255.0;
            }

            let index = i * 4;
            out[index] = (r * 255.0).clamp(0.0, 255.0) as u8;
            out[index + 1] = (g * 255.0).clamp(0.0, 255.0) as u8;
            out[index + 2] = (b * 255.0).clamp(0.0, 255.0) as u8;
            out[index + 3] = 255;
        }

        let info = ImageInfo::new(
            (width as i32, height as i32),
            ColorType::RGBA8888,
            AlphaType::Unpremul,
            None,
        );
        let result = images::raster_from_data(&info, Data::new_copy(&out), info.min_row_bytes())
            .ok_or_else(|| Error::ImageEncodeError("create image failed".to_string()))?;

        if lyric_on && !lyrics.is_empty() {
            let idx = rand::rng().random_range(0..lyrics.len());
            let (jp, cn) = &lyrics[idx];
            let mut surface = result.to_surface();
            overlay_lyric(
                surface.canvas(),
                width as i32,
                height as i32,
                &cover_image,
                jp,
                cn.as_deref(),
            );
            Ok(surface.image_snapshot())
        } else {
            Ok(result)
        }
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "louvre",
    louvre,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 0,
    keywords = &["卢浮宫"],
    date_created = local_date(2025, 5, 29),
    date_modified = local_date(2026, 10, 12),
);
