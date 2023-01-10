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
