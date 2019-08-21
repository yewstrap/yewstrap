use yew::prelude::*;

use crate::merge_classes;

#[derive(Properties)]
pub struct Props {
    pub class: String,
}

pub struct DropdownDivider {
    props: Props,
}

pub enum Msg {}

impl Component for DropdownDivider {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        DropdownDivider { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<DropdownDivider> for DropdownDivider {
    fn view(&self) -> Html<Self> {
        let classes: String = merge_classes("dropdown-divider", &self.props.class);

        html! {
            <div class=classes></div>
        }
    }
}
