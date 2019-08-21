pub mod col;
pub mod collapse;
pub mod container;
pub mod dropdown;
pub mod dropdown_divider;
pub mod dropdown_item;
pub mod dropdown_menu;
pub mod dropdown_toggler;
pub mod nav;
pub mod nav_item;
pub mod nav_link;
pub mod navbar;
pub mod navbar_brand;
pub mod navbar_toggler;
pub mod row;

pub fn merge_classes(class_a: &str, class_b: &str) -> String {
    format!("{} {}", class_a, class_b)
}

pub use self::col::Col;
pub use self::collapse::Collapse;
pub use self::container::Container;
pub use self::dropdown::Dropdown;
pub use self::dropdown_divider::DropdownDivider;
pub use self::dropdown_item::DropdownItem;
pub use self::dropdown_menu::DropdownMenu;
pub use self::dropdown_toggler::DropdownToggler;
pub use self::nav::Nav;
pub use self::nav_item::NavItem;
pub use self::nav_link::NavLink;
pub use self::navbar::Navbar;
pub use self::navbar_brand::NavbarBrand;
pub use self::navbar_toggler::NavbarToggler;
pub use self::row::Row;
