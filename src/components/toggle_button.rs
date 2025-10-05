use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ToggleButtonProps {
    #[props(default = 50)]
    pub width: i32,
    #[props(default = 50)]
    pub height: i32,
    #[props(default = 20)]
    pub radius: i32,
    #[props(default = "none".to_string())]
    pub fill: String,
    #[props(default = "black".to_string())]
    pub stroke: String,
    #[props(default = 1)]
    pub stroke_width: i8,
}
#[component]
pub fn ToggleButton(props: ToggleButtonProps) -> Element {
    rsx! {
        svg {
            class: "toggle-button",
            height: props.width.to_string(),
            width: props.width.to_string(),
            xmlns: "http://www.w3.org/2000/svg",
            circle {
                cx: (props.width / 2).to_string(),
                cy: (props.height / 2).to_string(),
                r: props.radius.to_string(),
            }
            line {
                x1: (props.width / 2).to_string(),
                y1: ((props.height / 2) - 10).to_string(),
                x2: (props.width / 2).to_string(),
                y2: ((props.height / 2) + 10).to_string(),
            }
        }
    }
}
