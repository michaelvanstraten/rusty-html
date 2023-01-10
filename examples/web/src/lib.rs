use rusty_html::html;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn time(a: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn timeEnd(a: &str);
}

// struct Counter {
//     // #[state]
//     count: Arc<Mutex<isize>>,
//     __element: Element,
// }
//
// impl Counter {
//     fn body(&self) {
//         self.__element.set_outer_html(&html! {
//             <div>
//                 The count is: {self.count.lock()?}
//             </div>
//         })
//     }
// }

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    time("document.create_element");
    let root_element = document.create_element("div")?;
    for _ in 0..100000 {
        let element = document.create_element("p")?;
        element.set_attribute("test", "test")?;
        root_element.append_child(&element)?;
    }
    body.append_child(&root_element)?;
    timeEnd("document.create_element");

    time("set_inner_html");
    let root_element = document.create_element("div")?;
    root_element.set_inner_html(&html! ({
        for _ in 0..100000 {
            <p test="test"></p>
        }
    }));
    body.append_child(&root_element)?;
    timeEnd("set_inner_html");

    Ok(())
}

