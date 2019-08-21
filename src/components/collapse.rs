use yew::prelude::*;

use crate::merge_classes;

#[derive(Properties)]
pub struct Props {
    pub is_open: bool,
    pub navbar: bool,

    pub class: String,
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
        let mut classes = String::from("collapse");

        if self.props.navbar {
            classes = merge_classes(&classes, "navbar-collapse");
        }

        if self.props.is_open {
            classes = merge_classes(&classes, "show");
        }

        classes = merge_classes(&classes, &self.props.class);

        html! {
            <div class=classes>
                { for (self.props.children).iter() }
            </div>
        }
    }
}
