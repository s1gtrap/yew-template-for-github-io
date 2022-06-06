use yew::prelude::*;

mod parent;

#[derive(Debug)]
pub struct Model {
    content: String,
    error: Option<Box<dyn std::error::Error>>,
    highlights: Vec<bhava::Interval<String>>,
    insts: Vec<dasha::Spanning<dasha::Inst>>,
}

pub enum Message {
    Change(String),
    MouseEnter(bhava::Interval<String>),
    MouseLeave(bhava::Interval<String>),
}

impl Model {
    fn disasm(&mut self) -> Result<Vec<dasha::Spanning<dasha::Inst>>, Box<dyn std::error::Error>> {
        let insts = dasha::disasm(dasha::text::tokenize(&self.content).unwrap())?;

        Ok(insts)
    }

    fn do_disasm(&mut self) {
        match self.disasm() {
            Ok(insts) => {
                self.insts = insts;
                self.error = None;
            }
            Err(err) => {
                self.insts = vec![];
                self.error = Some(err);
            }
        }
    }
}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let content = " 00 00   ";
        let mut this = Self {
            content: content.into(),
            error: None,
            highlights: vec![],
            insts: vec![],
        };
        this.do_disasm();
        this
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Change(t) => {
                log::info!("change {:?}", t);
                self.content = t.clone();
                self.do_disasm();
            }
            Message::MouseEnter(h) => self.highlights.push(h),
            Message::MouseLeave(h) => {
                let index = self.highlights.iter().position(|x| *x == h).unwrap();
                self.highlights.remove(index);
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        use dasha::Parent;
        html! {
            <div class="container my-5">
                <h3>
                    { "Dasha " }
                    <span class="lead">
                        { "@ " }
                        <a href="/">{ "https://disassemb.ly/" }</a>
                    </span>
                </h3>
                <p>
                    { "a [very] rudimentary x86 disassembler for the " }
                    <abbr title="targeting WebAssembly with wasm-bindgen bridging">{ "web" }</abbr>
                    { " written in Rust" }
                </p>
                <div class="row gx-2">
                    {
                        for self.error.iter().map(|err| html!(
                            <div class="alert alert-danger" role="alert">
                                { err }
                            </div>
                        ))
                    }
                </div>
                <div class="row gx-2">
                    <div class="col-md">
                        <div class="card mb-2">
                            <div class="card-header">
                                { "Binary" }
                            </div>

                            <bhava::Editor class="card-body font-monospace" highlights={self.highlights.clone()} content={self.content.clone()} on_change={ctx.link().callback(Message::Change)} />
                        </div>
                    </div>
                    <div class="col-md">
                        <div class="card mb-2">
                            <div class="card-header">
                                { "Assembly" }
                            </div>

                            <div class="card-body font-monospace">
                                {
                                    for self.insts.iter().map(|i @ &dasha::Spanning(_, s, l, _)| {
                                        let onmouseover = ctx.link().callback(move |_: MouseEvent| {
                                            log::info!("mouseenter");
                                            Message::MouseEnter(bhava::Interval(s, s + l, "scary".to_owned()))
                                        });
                                        let onmouseout = ctx.link().callback(move |_: MouseEvent| {
                                            log::info!("mouseleave");
                                            Message::MouseLeave(bhava::Interval(s, s + l, "scary".to_owned()))
                                        });
                                        html!(
                                            <div class="inst" {onmouseover} {onmouseout}>
                                                {
                                                    for i.children().iter().map(|i| html!(
                                                        <crate::parent::Parent parent={i.clone()} />
                                                    ))
                                                }
                                            </div>
                                        )
                                    })
                                }
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<Model>();
}
