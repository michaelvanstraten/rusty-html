use rusty_html::html;

fn main() {
    let html = html! {
        <html>
            <head>
            <title>Page Title</title>
            </head>
            <body>
            {
                for name in ["Tom", "Anna", "Bob"] {
                    <p>{name}</p>
                }
            }
            </body>
        </html>
    };
    println!("{}", html);
}
