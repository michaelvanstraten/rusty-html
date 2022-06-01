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
