use serde::Serialize;
use ts_rs::TS;

pub mod theming {
    use super::*;

    #[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
    struct LinuxWindowControls {
        #[serde(rename = "linux.windowCloseButtonBackground")]
        button_background: String,

        #[serde(rename = "linux.windowCloseButtonBackground.hovered")]
        button_background_hovered: String,

        #[serde(rename = "linux.windowCloseButtonBackground.focused")]
        button_background_focused: String,

        #[serde(rename = "linux.windowButton")]
        button: String,

        #[serde(rename = "linux.windowButton.hovered")]
        button_hovered: String,

        #[serde(rename = "linux.windowButton.focused")]
        button_focused: String,
    }

    #[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
    struct HeadBar {
        #[serde(rename = "headBar.background")]
        background: String,
    }

    #[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
    struct SideBar {
        #[serde(rename = "sideBar.background")]
        background: String,
    }

    #[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
    struct PageView {
        #[serde(rename = "pageView.background")]
        background: String,
    }

    #[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
    #[ts(export, export_to = "appearance.ts")]
    pub struct Colors {
        #[serde(rename = "primary")]
        primary: String,
        #[serde(flatten)]
        linux_window_controls: LinuxWindowControls,
        #[serde(flatten)]
        head_bar: HeadBar,
        #[serde(flatten)]
        side_bar: SideBar,
        #[serde(flatten)]
        page_view: PageView,
    }

    #[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
    #[serde(rename_all = "camelCase")]
    #[ts(export, export_to = "appearance.ts")]
    pub struct Theme {
        name: String,
        slug: String,
        #[serde(rename = "type")]
        typ: String,
        is_default: bool,
        colors: Colors,
    }
}
