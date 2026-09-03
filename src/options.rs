use meme_generator_utils::builder::MemeOptions;

#[derive(MemeOptions)]
pub(crate) struct NoOptions {}

macro_rules! number_option {
    ($name:ident, $min:tt, $max:tt) => {
        #[derive(MemeOptions)]
        struct $name {
            /// 图片编号
            #[option(short, long, minimum = $min, maximum = $max)]
            number: Option<i32>,
        }
    };
}

pub(crate) use number_option;
