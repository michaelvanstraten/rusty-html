use crate::render::Render;
use crate::render::RenderBuffer;

macro_rules! impl_render_for {
    ($integer_type:ty) => {
        impl<T: RenderBuffer> Render<T> for $integer_type {
            #[inline(always)]
            fn render_to_buf(&self, buf: &mut T) {
                buf.push_str(&self.to_string())
            }
        }
    };
    ($($integer_type:ty),*) => {
        $(impl_render_for!($integer_type);)*
    }
}

impl_render_for!(bool, char, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);

impl<T: RenderBuffer> Render<T> for () {
    #[inline]
    fn render_to_buf(&self, _buf: &mut T) {}
}
