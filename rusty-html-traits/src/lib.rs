#![feature(associated_type_bounds)]

extern crate rusty_html;
trait HTMLify {
    fn htmlify(&mut self) -> String;
}

impl HTMLify for &str {
    fn htmlify(&mut self) -> String {
        String::from_str(self).unwrap()
    }
}

//

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

use std::str::FromStr;

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

    #[test]
    fn it_works() {
        let test = "dsfsf";
        let test_var = true;
        let html = rusty_html::html!{
                <head>
                <title>
                    {
                       if (1 == 1) {
                           "sdffsdf"
                       } else {
                           "whaaa"
                       }
                    }
                </title>
                </head>
                <body>
                <h1>My First Heading</h1>
                <p>My first paragraph.</p>
                </body>
            }; 
        println!("{:}", html )
    }
}
