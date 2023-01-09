use rusty_html_macros::html;

#[test]
fn inline_html() {
    let id = 5;
    let html = html! {
        <body id={id}>
        {
            for index in 0..1 {
                <p>{index}</p>
            }
        }
        </body>
    };
    println!("{html}")
}
