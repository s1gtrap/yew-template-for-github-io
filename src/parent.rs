use yew::prelude::*;

#[derive(Debug)]
pub struct Parent;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub parent: dasha::Frag,
}

impl Component for Parent {
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
        log::info!("Parent::view");
        match &ctx.props().parent {
            dasha::Frag::Leaf(l) => html!({ l }),
            dasha::Frag::Branch(p) => html!(
                <span>
                    {
                        for p.children().iter().map(|p| html!(<Parent parent={p.clone()} />))
                    }
                </span>
            ),
        }
    }
}
