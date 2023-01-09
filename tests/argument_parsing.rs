use rusty_html_macros::template;

#[test]
fn parse_template_arguments() {
    #[template("./tests/templates/SimpleButton.html")]
    struct SimpleButton {}
}

