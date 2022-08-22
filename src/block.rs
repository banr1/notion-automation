use crate::bookmark::Bookmark;
use crate::bulleted_list_item::BulletedListItem;
use crate::child_database::ChildDatabase;
use crate::child_page::ChildPage;
use crate::code::Code;
use crate::divider::Divider;
use crate::embed::Embed;
use crate::equation::Equation;
use crate::heading::{Heading1, Heading2, Heading3};
use crate::image::Image;
use crate::link_preview::LinkPreview;
use crate::numbered_list_item::NumberedListItem;
use crate::paragraph::Paragraph;
use crate::synced_block::SyncedBlock;
use crate::table_of_contents::TableOfContents;
use crate::todo::ToDo;
use crate::toggle::Toggle;
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
    BulletedListItem(BulletedListItem),
    NumberedListItem(NumberedListItem),
    ToDo(ToDo),
    Toggle(Toggle),
    Code(Code),
    ChildPage(ChildPage),
    ChildDatabase(ChildDatabase),
    Embed(Embed),
    Image(Image),
    Video,
    File,
    Pdf,
    Bookmark(Bookmark),
    Callout,
    Quote,
    Equation(Equation),
    Divider(Divider),
    TableOfContents(TableOfContents),
    Column,
    ColumnList,
    LinkPreview(LinkPreview),
    SyncedBlock,
    Template,
    LinkToPage,
    Table,
    TableRow,
    Audio,
    Unsupported,
}
