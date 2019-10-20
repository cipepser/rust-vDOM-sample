extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;

pub enum VirtualNode {
    Text(&'static str),
    Element {
        tag_name: &'static str,
        children: Vec<VirtualNode>,
    },
}

pub fn render(document: &web_sys::Document, virtual_node: VirtualNode) -> web_sys::Node {
    match virtual_node {
        VirtualNode::Text(text) => document.create_text_node(text).into(),
        VirtualNode::Element { tag_name, children } => {
            let element = document.create_element(tag_name).unwrap();
            for child in children {
                element.append_child(&render(document, child)).unwrap();
            }
            element.into()
        }
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue>{
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let vdom = render(
        &document,
        VirtualNode::Element {
            tag_name: "h1",
            children: vec![VirtualNode::Text("Hello World")],
        },
    );

    body.append_child(&vdom)?;

    Ok(())
}