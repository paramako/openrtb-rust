use serde_repr::{Deserialize_repr, Serialize_repr};

/// This object describes the content in which the impression will appear, which may be syndicated
/// or non syndicated content.
///
/// This object may be useful when syndicated content contains impressions and does
/// not necessarily match the publisher’s general content.
///
/// The exchange might or might not have
/// knowledge of the page where the content is running, as a result of the syndication method.
///
/// For example might be a video impression embedded in an iframe on an unknown web property or device.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Content {
    /// ID uniquely identifying the content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Type of content
    /// ***
    /// 1 - `Video` (i.e., video file or stream such as Internet TV broadcasts)
    /// ***
    /// 2 - `Game` (i.e., an interactive software game)
    /// ***
    /// 3 - `Music` (i.e., audio file or stream such as Internet radio broadcasts)
    /// ***
    /// 4 - `Application` (i.e., an interactive software application)
    /// ***
    /// 5 - `Text` (i.e., primarily textual document such as a web page, eBook, or news article)
    /// ***
    /// 6 - `Other` (i.e., none of the other categories applies)
    /// ***
    /// 7 - `Unknown`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ContextType>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum ContextType {
    /// (i.e., video file or stream such as Internet TV broadcasts)
    Video = 1,
    /// (i.e., an interactive software game)
    Game = 2,
    /// (i.e., audio file or stream such as Internet radio broadcasts)
    Music = 3,
    /// (i.e., an interactive software application)
    Application = 4,
    /// (i.e., primarily textual document such as a web page, eBook, or news article)
    Text = 5,
    /// (i.e., none of the other categories applies)
    Other = 6,
    Unknown = 7,
}

#[test]
fn test_content_serde() {
    let expected = r#"{"id":"1234567","context":2}"#;

    let content = Content {
        id: Some("1234567".to_string()),
        context: Some(ContextType::Game),
    };

    let json = serde_json::to_string(&content).unwrap();

    assert_eq!(expected, json);
}
