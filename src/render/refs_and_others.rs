use std::borrow::Cow;
use std::cell::Ref;
use std::cell::RefMut;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::MutexGuard;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

use crate::render::Render;
use crate::render::RenderBuffer;

// derive from rust-sailfish/sailfish/sailfish/src/runtime/render.rs
macro_rules! impl_render_for {
    (
        [$($bounds:tt)+] $($desc:tt)+
    ) => {
        impl <$($bounds)+, B: RenderBuffer> Render<B> for $($desc)+ {
            #[inline]
            fn render_to_buf(&self, buf: &mut B) {
                (**self).render_to_buf(buf)
            }

            #[inline]
            fn render_to_buf_escaped(&self, buf: &mut B) {
                (**self).render_to_buf_escaped(buf)
            }
        }
    };
}

impl_render_for!(['a, T: Render<B> + ?Sized] &'a T);
impl_render_for!(['a, T: Render<B> + ?Sized] &'a mut T);
impl_render_for!([T: Render<B> + ?Sized] Box<T>);
impl_render_for!([T: Render<B> + ?Sized] Rc<T>);
impl_render_for!([T: Render<B> + ?Sized] Arc<T>);
impl_render_for!(['a, T: Render<B> + ToOwned + ?Sized] Cow<'a, T>);
impl_render_for!(['a, T: Render<B> + ?Sized] Ref<'a, T>);
impl_render_for!(['a, T: Render<B> + ?Sized] RefMut<'a, T>);
impl_render_for!(['a, T: Render<B> + ?Sized] MutexGuard<'a, T>);
impl_render_for!(['a, T: Render<B> + ?Sized] RwLockReadGuard<'a, T>);
impl_render_for!(['a, T: Render<B> + ?Sized] RwLockWriteGuard<'a, T>);
