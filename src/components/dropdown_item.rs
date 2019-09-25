use yew::html::Children;
use yew::prelude::*;

#[derive(Properties)]
pub struct Props {
    pub href: String,

    pub class: Classes,
    pub children: Children<DropdownItem>,
}

pub struct DropdownItem {
    props: Props,
}

pub enum Msg {}

impl Component for DropdownItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        DropdownItem { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<DropdownItem> for DropdownItem {
    fn view(&self) -> Html<Self> {
        let classes = self.props.class.clone().extend("dropdown-item");

        let mut href = String::from(&self.props.href);

        if href == "" {
            href = format!("#");
        }

        html! {
            <a class=classes href=href>
                { for (self.props.children).iter() }
            </a>
        }
    }
}
