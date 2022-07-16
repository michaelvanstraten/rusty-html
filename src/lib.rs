/*!
 This crate allows for Jsx html inline like syntax in rust

 ## Example
```rust
use rusty_html::html;

fn main() {
    let html = html! {
        <html>
            <head>
            <title>Page Title</title>
            </head>
            <body>
            {
                vec!["ad", "sdf", "sdfsdf"].into_iter().map(|s| {
                        html!{
                            <p>{s}</p>
                        }
                    }
                ).collect::<Vec<String>>()
            }
            <h1>sfsdf</h1>
            <p>My first paragraph.</p>
            </body>
        </html>
    };
    println!("{}", html);
}
```
 */

#![feature(fn_traits)]

pub use rusty_html_macros::html;

pub trait HTMLify {
    fn htmlify(&self) -> String;
}

macro_rules! impl_for {
    ($($type:ty),+) => {
        $(
            impl HTMLify for $type {
                fn htmlify(&self) -> String {
                    self.to_string()
                }
            }
        )+
    }
}

impl_for! {
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    f32,
    f64,
    char,
    &str,
    bool
}

impl HTMLify for String {
    fn htmlify(&self) -> String {
        self.to_owned()
    }
}

impl<T: HTMLify> HTMLify for Vec<T> {
    fn htmlify(&self) -> String {
        let mut new_string = String::new();
        for s in self {
            new_string.push(' ');
            new_string.push_str(&s.htmlify())
        }
        new_string
    }
}

#[cfg(test)]
#[test]
fn test() {
    let a = "Hello";
    let b = a.to_owned();

    let c = |n: String| {
        format!("{a}, {n}")
    };

    println!("{}", c.call(("Peter".to_owned(),)));
}