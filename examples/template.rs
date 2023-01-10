use rusty_html::Template;
use rusty_html::render::Render;

#[Template("./examples/templates/SimpleLink.html")]
struct SimpleLink {
    link: &'static str,
    link_title: &'static str,
}

fn main() {
    let mut html = String::new();

    SimpleLink {
        link: "https://example.com",
        link_title: "Example"
    }.render_to_buf(&mut html);

    println!("{html}")
}
