use yew::prelude::*;

pub struct Dropdown {
    show: bool,
}

pub enum Msg {
    Toggle,
}

impl Component for Dropdown {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Dropdown { show: false }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggle => {
                self.show = !self.show;
            }
        }
        true
    }
}

impl Renderable<Dropdown> for Dropdown {
    fn view(&self) -> Html<Self> {
        let mut dropdown_menu_classes = String::from("dropdown-menu");

        if self.show {
            dropdown_menu_classes = format!("{} show", dropdown_menu_classes);
        }

        html! {
            <li class="nav-item dropdown">
                <a class="nav-link dropdown-toggle" href="#" id="navbarDropdown" role="button" data-toggle="dropdown" aria-haspopup="true" aria-expanded="false" onclick=|_| Msg::Toggle >
                {"Dropdown"}
                </a>
                <div class={dropdown_menu_classes} aria-labelledby="navbarDropdown">
                    <a class="dropdown-item" href="#">{"Action"}</a>
                    <a class="dropdown-item" href="#">{"Another action"}</a>
                    <div class="dropdown-divider"></div>
                    <a class="dropdown-item" href="#">{"Something else here"}</a>
                </div>
            </li>
        }
    }
}
