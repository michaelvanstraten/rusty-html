mod impls;

pub trait Render<T: RenderBuffer> {
    fn render_to_buf(&self, buf: &mut T);

    fn render_to_buf_escaped(&self, buf: &mut T) {
        self.render_to_buf(buf)
    }
}

pub trait RenderBuffer {
    fn with_capacity(capacity: usize) -> Self;
    fn push_str(&mut self, string: &str);
}

impl RenderBuffer for String {
    fn with_capacity(capacity: usize) -> Self {
        String::with_capacity(capacity)
    }

    fn push_str(&mut self, string: &str) {
        self.push_str(string)
    }
}
