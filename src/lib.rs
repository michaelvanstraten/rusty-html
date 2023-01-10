/*!
Rusty HTML is a html-templating/web(in progress) framework.

## Example

```rust
use rusty_html::html;

fn main() {
    let links = [
        ("https://wikipedia.com", "Wikipedia"),
        ("https://twitter.com", "Twitter"),
        ("https://youtube.com", "Youtube"),
    ];

    let html = html!(
        <div>
        {
            for (link, link_title) in links {
                <a src={link}>{link_title}</a>
            }
        }
        </div>
    );

    println!("{html}");
}
```

License: MIT
*/

pub mod render;

pub use rusty_html_macros::html;
pub use rusty_html_macros::template as Template;
