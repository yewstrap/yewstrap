use yew::prelude::*;

#[derive(Properties)]
pub struct Props {
    pub class: Classes,
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
        let classes = self.props.class.extend("dropdown-divider");

        html! {
            <div class=classes></div>
        }
    }
}
