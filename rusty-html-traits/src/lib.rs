#![feature(associated_type_bounds)]

trait HTMLify {
    fn htmlify(&mut self) -> String;
}

// impl<T : std::convert::Into<String>> HTMLify for T {
//     fn htmlify(self) -> String {
//         self.into()
//     }
// }

impl HTMLify for i64 {
    fn htmlify(&mut self) -> String {
        self.into()
    }
}


impl<T : Iterator<Item : HTMLify>> HTMLify for T {
    fn htmlify(&mut self) -> String {
        let mut new_string = String::new();
        for ref mut item in self {
            new_string.push_str(&item.htmlify())
        }
        new_string
    }
}


// impl<T : Iterator<Item : HTMLify>> HTMLify for T {
//     fn htmlify(&mut self) -> String {
//         let mut new_string = String::new();
//         for ref mut item in self {
//             new_string.push_str(&item.htmlify())
//         }
//         new_string
//     }
// }

// impl<T : ToString> HTMLify for T {
//     fn to_html(&self) -> String {
//         self.to_string()
//     }
// }

// impl HTML for [&str] {
//     fn to_html(&self) -> String {
//         let mut new_string = String::new();
//         for str in self {
//             new_string.push_str(*str);
//             new_string.push_str("\n");
//         }
//         new_string
//     }
// }

// impl HTML for Vec<e&str> {
//     fn to_html(&self) -> String {
//         let mut new_string = String::new();
//         for str in self {
//             new_string.push_str(&str.to_html());
//             new_string.push('\n');
//         }
//         new_string
//     }
// }
#[cfg(test)]
mod tests {
    use crate::HTMLify;
    // use rusty_html::html;
    use rusty_html::html;
    #[test]
    fn it_works() {
        let html = html!{
            <html>
                <head>
                <title test = {vec!["ad", "sdf", "sdfsdf"]}>Page Title</title>
                </head>
                <body>
                <h1>My First Heading</h1>
                <p>My first paragraph.</p>
                </body>
            </html>
        };
        println!("{}", html);
    }
}
