use crate::heading::{Heading1, Heading2, Heading3};
use crate::image::Image;
use crate::paragraph::Paragraph;
use crate::synced_block::SyncedBlock;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Block {
    // https://developers.notion.com/reference/block
    Paragraph(Paragraph),
    #[serde(rename = "heading_1")]
    Heading1(Heading1),
    #[serde(rename = "heading_2")]
    Heading2(Heading2),
    #[serde(rename = "heading_3")]
    Heading3(Heading3),
    BulletedListItem,
    NumberedListItem,
    ToDo,
    Toggle,
    ChildPage,
    ChildDatabase,
    Image(Image),
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
    SyncedBlock(SyncedBlock),
    Template,
    LinkToPage,
    Table,
    TableRow,
    Unsupported,
}
