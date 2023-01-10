use crate::render::Render;
use crate::render::RenderBuffer;

impl<T: RenderBuffer> Render<T> for String {
    #[inline(always)]
    fn render_to_buf(&self, buf: &mut T) {
        buf.push_str(self)
    }

    // fn render_to_buf_escaped(&self, _buf: &mut T) {
    //     todo!()
    // }
}

impl<T: RenderBuffer> Render<T> for str {
    fn render_to_buf(&self, buf: &mut T) {
        buf.push_str(self)
    }

    // fn render_to_buf_escaped(&self, _buf: &mut T) {
    //     todo!()
    // }
}

