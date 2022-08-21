use crate::color::Color;
use crate::icon::FileContent;
use crate::rich_text::RichText;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Block {
    // https://developers.notion.com/reference/block
    Paragraph {
        id: String,
        has_children: bool,
        paragraph: ParagraphContent,
    },
    #[serde(rename = "heading_1")]
    Heading1 {
        id: String,
        has_children: bool,
        heading_1: HeadingContent,
    },
    #[serde(rename = "heading_2")]
    Heading2 {
        id: String,
        has_children: bool,
        heading_2: HeadingContent,
    },
    #[serde(rename = "heading_3")]
    Heading3 {
        id: String,
        has_children: bool,
        heading_3: HeadingContent,
    },
    BulletedListItem,
    NumberedListItem,
    ToDo,
    Toggle,
    ChildPage,
    ChildDatabase,
    Embed,
    Image {
        id: String,
        has_children: bool,
        image: FileContent,
    },
    Video,
    File,
    Pdf,
    Bookmark,
    Callout,
    Quote,
    Equation,
    Divider,
    TableOfContents,
    Column,
    ColumnList,
    LinkPreview,
    SyncedBlock {
        id: String,
        has_children: bool,
        synced_block: SyncedBlockContent,
    },
    Template,
    LinkToPage,
    Table,
    TableRow,
    Unsupported,
}

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct ParagraphContent {
    color: Color,
    rich_text: Option<Vec<RichText>>,
    // children: Option<Vec<Object>>,
}

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct HeadingContent {
    color: Color,
    rich_text: Option<Vec<RichText>>,
}

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct SyncedBlockContent {
    synced_from: Option<String>,
    rich_text: Option<Vec<RichText>>,
}
