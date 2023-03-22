use serde_json::Value;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    pub text: Option<String>,
    #[serde(default = "bool::default")]
    pub bold: bool,
    #[serde(default = "bool::default")]
    pub italic: bool,
    #[serde(default = "bool::default")]
    pub underlined: bool,
    #[serde(default = "bool::default")]
    pub strikethrough: bool,
    #[serde(default = "bool::default")]
    pub obfuscated: bool,
    #[serde(default = "default_font")]
    pub font: String,
    #[serde(default = "default_color")]
    pub color: String,
    pub insertion: Option<String>,
    pub click_event: Option<ClickEvent>,
    pub hover_event: Option<HoverEvent>,
    pub extra: Option<Vec<Chat>>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum ClickEvent {
    OpenUrl(#[serde(rename = "value")] String),
    RunCommand(#[serde(rename = "value")] String),
    SuggestCommand(#[serde(rename = "value")] String),
    ChangePage(#[serde(rename = "value")] i32),
    CopyToClipboard(#[serde(rename = "value")] String),
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum HoverEvent {
    ShowText(#[serde(rename = "value")] Value),
}

fn default_font() -> String {
    "minecraft:default".to_string()
}

fn default_color() -> String {
    "\u{00A7}7".to_string()
}

#[test]
fn should_deserialize() {
    let raw = r#"
    {
        "text": "foo",
        "bold": true,
        "extra": [
            {
                "text": "bar"
            },
            {
                "text": "baz",
                "bold": false
            },
            {
                "text": "qux",
                "bold": true
            }
        ]
    }
    "#;

    let chat: Chat = serde_json::from_str(&raw).unwrap();
    dbg!(&chat);
}
