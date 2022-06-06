use yew::prelude::*;

#[derive(Debug)]
pub struct Inst;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub inst: dasha::Spanning<dasha::Inst>,
}

impl Component for Inst {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        unimplemented!()
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        for c in ctx.props().inst.children() {
            log::info!("def {}", c);
        }
        html! {
                {
                    for ctx.props().inst.children().iter().map(|i| html!(
                        <crate::parent::Parent parent={i.clone()} />
                    ))
                }
        }
    }
}
