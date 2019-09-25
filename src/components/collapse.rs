use yew::prelude::*;

#[derive(Properties)]
pub struct Props {
    pub is_open: bool,
    pub navbar: bool,

    pub class: Classes,
    pub children: Children<Collapse>,
}

pub struct Collapse {
    props: Props,
}

pub enum Msg {}

impl Component for Collapse {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Collapse { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<Collapse> for Collapse {
    fn view(&self) -> Html<Self> {
        let mut classes = self.props.class.clone().extend("collapse");

        if self.props.navbar {
            classes.push("navbar-collapse");
        }

        if self.props.is_open {
            classes.push("show");
        }

        html! {
            <div class=classes>
                { for (self.props.children).iter() }
            </div>
        }
    }
}
