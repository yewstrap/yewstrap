use yew::prelude::*;

#[derive(Properties)]
pub struct Props {
    pub is_open: bool,

    pub class: Classes,
    pub children: Children<DropdownMenu>,
}

pub struct DropdownMenu {
    props: Props,
}

pub enum Msg {}

impl Component for DropdownMenu {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        DropdownMenu { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<DropdownMenu> for DropdownMenu {
    fn view(&self) -> Html<Self> {
        let mut classes = self.props.class.clone().extend("dropdown-menu");

        if self.props.is_open {
            classes.push("show");
        }

        html! {
            <div class=classes  aria-labelledby="navbarDropdown">
                { for (self.props.children).iter() }
            </div>
        }
    }
}
