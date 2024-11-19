pub mod theming {
    use ts_rs::TS;
    use std::string::ParseError;
    use serde::{Serialize, Deserialize, Deserializer};
    use super::*;


    #[derive(Deserialize,Serialize, Debug, Clone, Eq, PartialEq, TS)]
    struct LinuxWindowControls {
        #[serde(rename = "windowControlsLinux.background", deserialize_with = "transform_to_rgba")]
        #[ts(rename = "windowControlsLinux.background")]
        background: String,

        #[serde(rename = "windowControlsLinux.text", deserialize_with = "transform_to_rgba")]
        #[ts(rename = "windowControlsLinux.text")]
        text: String,

        #[serde(rename = "windowControlsLinux.hoverBackground", deserialize_with = "transform_to_rgba")]
        #[ts(rename = "windowControlsLinux.hoverBackground")]
        hover_background: String,

        #[serde(rename = "windowControlsLinux.activeBackground", deserialize_with = "transform_to_rgba")]
        #[ts(rename = "windowControlsLinux.activeBackground")]
        active_background: String,
    }

    #[derive(Deserialize,Serialize, Debug, Clone, Eq, PartialEq, TS)]
    struct WindowsWindowControls {
        #[serde(rename = "windowsCloseButton.background", deserialize_with = "transform_to_rgba")]
        #[ts(rename = "windowsCloseButton.background")]
        close_button_background: String,
    }

    #[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
    struct HeadBar {
        #[serde(rename = "headBar.background", deserialize_with = "transform_to_rgba")]
        #[ts(rename = "headBar.background")]
        background: String,
    }

    #[derive(Deserialize,Serialize, Debug, Clone, Eq, PartialEq, TS)]
    struct SideBar {
        #[serde(rename = "sideBar.background", deserialize_with = "transform_to_rgba")]
        #[ts(rename = "sideBar.background")]
        background: String,
    }

    #[derive(Deserialize,Serialize, Debug, Clone, Eq, PartialEq, TS)]
    struct ToolBar {
        #[serde(rename = "toolBar.background", deserialize_with = "transform_to_rgba")]
        #[ts(rename = "toolBar.background")]
        background: String,
    }

    #[derive(Deserialize,Serialize, Debug, Clone, Eq, PartialEq, TS)]
    struct Page {
        #[serde(rename = "page.background", deserialize_with = "transform_to_rgba")]
        #[ts(rename = "page.background")]
        background: String,
    }

    #[derive(Deserialize,Serialize, Debug, Clone, Eq, PartialEq, TS)]
    struct StatusBar {
        #[serde(rename = "statusBar.background", deserialize_with = "transform_to_rgba")]
        #[ts(rename = "statusBar.background")]
        background: String,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, TS)]
    #[ts(export, export_to = "appearance.ts")]
    pub struct Colors {
        #[serde(rename = "primary", deserialize_with = "transform_to_rgba")]
        primary: String,

        #[serde(flatten)]
        side_bar: SideBar,

        #[serde(flatten)]
        tool_bar: ToolBar,

        #[serde(flatten)]
        page: Page,

        #[serde(flatten)]
        status_bar: StatusBar,

        #[serde(flatten)]
        windows_window_controls: WindowsWindowControls,

        #[serde(flatten)]
        linux_window_controls: LinuxWindowControls,

    }

    #[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, TS)]
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

    fn transform_to_rgba<'de, D>(deserializer: D) -> Result<String, D::Error>
    where D: Deserializer<'de>
    {
        let content = String::deserialize(deserializer)?;
        let [r, g, b, a] = csscolorparser::parse(content.as_str())
            .unwrap()
            .to_rgba8();
        let result = vec![r.to_string(),
             g.to_string(),
             b.to_string(),
             format!("{:.2}", a as f64 / 255.0)];
        Ok(result.join(","))

    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_rgba_theme() {
            let theme = r###"
{
  "name": "Moss Pink",
  "slug": "moss-pink",
  "type": "pink",
  "isDefault": false,
  "colors": {
    "primary": "rgba(0, 0, 0, 1)",
    "sideBar.background": "rgba(234, 157, 242, 1)",
    "toolBar.background": "rgba(222, 125, 232, 1)",
    "page.background": "rgba(227, 54, 245, 1)",
    "statusBar.background": "rgba(63, 11, 69, 1)",
    "windowsCloseButton.background": "rgba(196, 43, 28, 1)",
    "windowControlsLinux.background": "rgba(218, 218, 218, 1)",
    "windowControlsLinux.text": "rgb(61, 61, 61, 1)",
    "windowControlsLinux.hoverBackground": "rgb(209, 209, 209, 1)",
    "windowControlsLinux.activeBackground": "rgb(191, 191, 191, 1)"
  }
}
"###;
            let theme: Theme = serde_json::from_str(theme).unwrap();
            println!("{:#?}", theme);
        }

        #[test]
        fn test_hex_theme () {
            let theme = r###"
{
  "name": "Moss Pink",
  "slug": "moss-pink",
  "type": "pink",
  "isDefault": false,
  "colors": {
    "primary": "#000000",
    "sideBar.background": "#000000",
    "toolBar.background": "#0000FF",
    "page.background": "#0000FF",
    "statusBar.background": "#00FF00",
    "windowsCloseButton.background": "#00FF00",
    "windowControlsLinux.background": "#FF0000",
    "windowControlsLinux.text": "#FF0000",
    "windowControlsLinux.hoverBackground": "#FFFFFF",
    "windowControlsLinux.activeBackground": "#FFFFFF"
  }
}
"###;
            let theme: Theme = serde_json::from_str(theme).unwrap();
            println!("{:#?}", theme);
        }

        #[test]
        fn test_hsl_theme() {
            let theme = r###"
{
  "name": "Moss Pink",
  "slug": "moss-pink",
  "type": "pink",
  "isDefault": false,
  "colors": {
    "primary": "hsl(0, 100%, 50%)",
    "sideBar.background": "hsl(120, 100%, 50%)",
    "toolBar.background": "hsl(240, 100%, 50%)",
    "page.background": "hsl(0, 0%, 0%)",
    "statusBar.background": "hsl(0, 0%, 50%)",
    "windowsCloseButton.background": "hsl(0, 0%, 100%)",
    "windowControlsLinux.background": "hsl(0, 50%, 50%)",
    "windowControlsLinux.text": "hsl(120, 50%, 50%)",
    "windowControlsLinux.hoverBackground": "hsl(240, 50%, 50%)",
    "windowControlsLinux.activeBackground": "hsl(360, 50%, 50%)"
  }
}
"###;
            let theme: Theme = serde_json::from_str(theme).unwrap();
            println!("{:#?}", theme);
        }
    }
}
