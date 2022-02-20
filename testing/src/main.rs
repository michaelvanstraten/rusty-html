use rusty_html::html;


fn main() {
    let html = html!{
        {235}
        {235}
        {235}
        {235}
        {235}
        {235}
        <html>
            <head>
            <title style={"color : blue"}>Page Title</title>
            </head>
            <body>
                {
                    "sfsdfsd"
                }
            <h1>My First Heading</h1>
            <p>My first paragraph.</p>

            </body>
        </html>
        {123123}
        {"sdfsdf"}
    };
    println!("{}", html);
}
