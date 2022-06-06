use std::collections::{HashSet, VecDeque};

use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged, Dispatched, Dispatcher};

pub struct Editor {
    text: String,
    error: Option<Box<dyn std::error::Error>>,
    node_ref: NodeRef,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub class: String,
    pub highlights: Vec<(usize, usize, Option<u8>)>,
    pub onchange: Callback<String>,
}

pub enum Message {
    Input,
    Hover(Option<(usize, usize)>),
}

impl Component for Editor {
    type Message = Message;
    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Editor {
            text: String::new(),
            error: None,
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Input => {
                let text = self
                    .node_ref
                    .cast::<web_sys::HtmlElement>()
                    .unwrap()
                    .inner_text();
                ctx.props().onchange.emit(text);
                return false;
            }
            Message::Hover(hl) => {
                log::info!("set hl {:?}", hl);
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        log::info!("{:?}", ctx.props().highlights);
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let sel = window.get_selection().unwrap().unwrap();
        let range = sel.get_range_at(0).unwrap();
        range.collapse_with_to_start(true);

        // get cursor index
        let nodes = self.node_ref.cast::<web_sys::Node>().unwrap().child_nodes();
        let mut start = 0;
        for i in 0..nodes.length() {
            use wasm_bindgen::JsCast;
            let node = nodes.get(i).unwrap();
            if node == range.start_container().unwrap() {
                break;
            }
            match node.node_type() {
                web_sys::Node::ELEMENT_NODE => {
                    let element = node.dyn_ref::<web_sys::HtmlElement>().unwrap();
                    start += element.inner_text().len() as u32;
                }
                web_sys::Node::TEXT_NODE => {
                    let text = node.dyn_ref::<web_sys::Text>().unwrap();
                    log::info!("{:?}", text.whole_text().unwrap());
                    start += text.whole_text().unwrap().len() as u32;
                }
                n => unimplemented!("nodeType == {}", n),
            }
        }
        log::info!("{}, {}", start, range.start_offset().unwrap());
        start += range.start_offset().unwrap();

        return false;
        // clear element
        let html_element = self.node_ref.cast::<web_sys::HtmlElement>().unwrap();
        html_element.set_inner_html("");

        let text = self.text.clone();
        match ctx.props().highlights.get(0) {
            Some((s, l, _)) => {
                use wasm_bindgen::JsCast;
                let (pre, hl) = text.split_at(text.char_indices().nth(*s).unwrap().0);
                let (hl, post) = hl.split_at(
                    text.char_indices()
                        .nth(*l)
                        .map(|(i, _)| i)
                        .unwrap_or(text.len()),
                );
                let tn = document.create_text_node(pre);
                html_element.append_child(&tn).unwrap();
                let hle = document.create_element("span").unwrap();
                let hle = hle.dyn_ref::<web_sys::HtmlElement>().unwrap();
                hle.class_list().add_2("text-white", "bg-dark").unwrap();
                hle.set_inner_text(hl);
                html_element.append_child(&hle).unwrap();
                let tn = document.create_text_node(post);
                html_element.append_child(&tn).unwrap();
            }
            None => {
                html_element.set_inner_text(&text);
            }
        }

        // recover cursor
        let nodes = self.node_ref.cast::<web_sys::Node>().unwrap().child_nodes();
        let mut offset = 0;
        for i in 0..nodes.length() {
            use wasm_bindgen::JsCast;
            let node = nodes.get(i).unwrap();
            match node.node_type() {
                web_sys::Node::ELEMENT_NODE => {
                    let element = node.dyn_ref::<web_sys::HtmlElement>().unwrap();
                    if offset <= start && start <= offset + element.inner_text().len() as u32 {
                        log::info!("cursor is in element {}", i);
                    }
                    offset += element.inner_text().len() as u32;
                }
                web_sys::Node::TEXT_NODE => {
                    let text = node.dyn_ref::<web_sys::Text>().unwrap();
                    log::info!("{:?}", text.whole_text().unwrap());
                    if offset <= start && start <= offset + text.whole_text().unwrap().len() as u32
                    {
                        log::info!("cursor is in element {}, setting to {}", i, start);
                        let sel = window.get_selection().unwrap().unwrap();
                        sel.remove_all_ranges().unwrap();
                        let range = document.create_range().unwrap();
                        range.set_start(&node, start - offset).unwrap();
                        sel.add_range(&range).unwrap();
                    }
                    offset += text.whole_text().unwrap().len() as u32;
                }
                n => unimplemented!("nodeType == {}", n),
            }
        }

        return false;

        let mut html = self.text.clone();
        if let Some((s, l, _)) = ctx.props().highlights.get(0) {
            html.insert_str(*s, "\"");
            html.insert_str(s + l + "\"".len(), "\"");
        }

        //log::info!("{}", range.start_offset().unwrap());
        self.node_ref
            .cast::<web_sys::HtmlElement>()
            .unwrap()
            .set_inner_text(&html);
        let sel = window.get_selection().unwrap().unwrap();
        sel.remove_all_ranges().unwrap();
        let range = document.create_range().unwrap();
        let nodes = self.node_ref.cast::<web_sys::Node>().unwrap().child_nodes();
        let text = self
            .node_ref
            .cast::<web_sys::Node>()
            .unwrap()
            .child_nodes()
            .item(0)
            .unwrap();
        let mut offset = 0;
        for i in 0..nodes.length() {
            use wasm_bindgen::JsCast;
            let node = nodes.get(i).unwrap();
            match node.node_type() {
                web_sys::Node::ELEMENT_NODE => {
                    let element = node.dyn_ref::<web_sys::HtmlElement>().unwrap();
                    offset += element.inner_text().len();
                }
                web_sys::Node::TEXT_NODE => {
                    let text = node.dyn_ref::<web_sys::Text>().unwrap();
                    log::info!("{:?}", text.whole_text().unwrap());
                    offset += text.whole_text().unwrap().len();
                }
                n => unimplemented!("nodeType == {}", n),
            }
        }

        range.set_start(&text, start).unwrap();
        sel.add_range(&range).unwrap();
        //log::info!("{}", range.start_offset().unwrap());
        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let oninput = ctx.link().callback(|_| Message::Input);
        //let oninput = ctx.props().onchange.reform(
        let mut html = self.text.clone();
        if let Some((s, l, _)) = ctx.props().highlights.get(0) {
            html.insert_str(*s, " ");
            html.insert_str(s + l + "\"".len(), " ");
        }
        log::info!("Editor::view {:?}", html);
        html! {
            <div class={&ctx.props().class} ref={self.node_ref.clone()} contenteditable="true" {oninput}></div>
        }
    }
}
