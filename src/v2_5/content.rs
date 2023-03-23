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
    pub context: Option<ContentContextType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContentContextType {
    /// (i.e., video file or stream such as Internet TV broadcasts)
    Video,
    /// (i.e., an interactive software game)
    Game,
    /// (i.e., audio file or stream such as Internet radio broadcasts)
    Music,
    /// (i.e., an interactive software application)
    Application,
    /// (i.e., primarily textual document such as a web page, eBook, or news article)
    Text,
    /// (i.e., none of the other categories applies)
    Other,
    Unknown,
}

impl From<ContentContextType> for i32 {
    fn from(value: ContentContextType) -> i32 {
        match value {
            ContentContextType::Video => 1,
            ContentContextType::Game => 2,
            ContentContextType::Music => 3,
            ContentContextType::Application => 4,
            ContentContextType::Text => 5,
            ContentContextType::Other => 6,
            ContentContextType::Unknown => 7,
        }
    }
}
