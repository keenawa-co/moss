pub mod theming {
    use serde::{Deserialize, Deserializer, Serialize};
    use ts_rs::TS;

    #[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, TS)]
    #[ts(export, export_to = "appearance.ts")]
    pub enum ThemeType {
        #[serde(rename = "light")]
        Light,
        #[serde(rename = "dark")]
        Dark,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, TS)]
    struct LinuxWindowControls {
        #[serde(
            rename = "windowControlsLinux.background",
            deserialize_with = "transform_to_rgba"
        )]
        #[ts(rename = "windowControlsLinux.background")]
        background: String,

        #[serde(
            rename = "windowControlsLinux.text",
            deserialize_with = "transform_to_rgba"
        )]
        #[ts(rename = "windowControlsLinux.text")]
        text: String,

        #[serde(
            rename = "windowControlsLinux.hoverBackground",
            deserialize_with = "transform_to_rgba"
        )]
        #[ts(rename = "windowControlsLinux.hoverBackground")]
        hover_background: String,

        #[serde(
            rename = "windowControlsLinux.activeBackground",
            deserialize_with = "transform_to_rgba"
        )]
        #[ts(rename = "windowControlsLinux.activeBackground")]
        active_background: String,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, TS)]
    struct WindowsWindowControls {
        #[serde(
            rename = "windowsCloseButton.background",
            deserialize_with = "transform_to_rgba"
        )]
        #[ts(rename = "windowsCloseButton.background")]
        close_button_background: String,
    }

    #[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
    struct HeadBar {
        #[serde(rename = "headBar.background", deserialize_with = "transform_to_rgba")]
        #[ts(rename = "headBar.background")]
        background: String,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, TS)]
    struct SideBar {
        #[serde(rename = "sideBar.background", deserialize_with = "transform_to_rgba")]
        #[ts(rename = "sideBar.background")]
        background: String,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, TS)]
    struct ToolBar {
        #[serde(rename = "toolBar.background", deserialize_with = "transform_to_rgba")]
        #[ts(rename = "toolBar.background")]
        background: String,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, TS)]
    struct Page {
        #[serde(rename = "page.background", deserialize_with = "transform_to_rgba")]
        #[ts(rename = "page.background")]
        background: String,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, TS)]
    struct StatusBar {
        #[serde(
            rename = "statusBar.background",
            deserialize_with = "transform_to_rgba"
        )]
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
    where
        D: Deserializer<'de>,
    {
        let content = String::deserialize(deserializer)?;
        let [r, g, b, a] = csscolorparser::parse(content.as_str()).unwrap().to_rgba8();
        let result = vec![
            r.to_string(),
            g.to_string(),
            b.to_string(),
            format!("{:.2}", a as f64 / 255.0),
        ];
        Ok(result.join(","))
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use serde::de::value::{Error, StrDeserializer};
        use serde::de::IntoDeserializer;
        #[test]
        fn test_parsing_correctly_formatted_rgba() {
            let value: StrDeserializer<Error> = "rgba(1, 2, 3, 1)".into_deserializer();
            assert_eq!(transform_to_rgba(value).unwrap(), "1,2,3,1.00".to_string())
        }

        #[test]
        fn test_parsing_correctly_formatted_rgb() {
            let value: StrDeserializer<Error> = "rgb(1, 2, 3)".into_deserializer();
            assert_eq!(transform_to_rgba(value).unwrap(), "1,2,3,1.00".to_string())
        }

        #[test]
        fn test_parsing_correctly_formatted_hex() {
            let value: StrDeserializer<Error> = "#010203".into_deserializer();
            assert_eq!(transform_to_rgba(value).unwrap(), "1,2,3,1.00".to_string())
        }
        #[test]
        fn test_parsing_correctly_formatted_hsl() {
            let value: StrDeserializer<Error> = "hsl(0, 100%, 50%)".into_deserializer();
            assert_eq!(
                transform_to_rgba(value).unwrap(),
                "255,0,0,1.00".to_string()
            )
        }

        #[test]
        fn test_parsing_without_spaces() {
            let value: StrDeserializer<Error> = "rgba(1,2,3,1)".into_deserializer();
            assert_eq!(transform_to_rgba(value).unwrap(), "1,2,3,1.00".to_string())
        }

        #[test]
        fn test_parsing_missing_alpha() {
            let value: StrDeserializer<Error> = "rgba(0, 0, 0)".into_deserializer();
            assert_eq!(transform_to_rgba(value).unwrap(), "0,0,0,1.00".to_string())
        }

        #[test]
        fn test_parsing_extra_alpha() {
            let value: StrDeserializer<Error> = "rgb(0, 0, 0, 1)".into_deserializer();
            transform_to_rgba(value).unwrap();
            assert_eq!(transform_to_rgba(value).unwrap(), "0,0,0,1.00".to_string())
        }

        #[test]
        #[should_panic]
        fn test_parsing_empty_color() {
            let value: StrDeserializer<Error> = "".into_deserializer();
            transform_to_rgba(value).unwrap();
        }

        #[test]
        #[should_panic]
        fn test_parsing_invalid_rgba() {
            let value: StrDeserializer<Error> = "rgba(-1, 0, 0, 1)".into_deserializer();
            transform_to_rgba(value).unwrap();
        }

        #[test]
        #[should_panic]
        fn test_parsing_invalid_hex() {
            let value: StrDeserializer<Error> = "#gggggg".into_deserializer();
            transform_to_rgba(value).unwrap();
        }
    }
}
