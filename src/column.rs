use std::fmt;
use strum::EnumIter;

use serde::Serialize;

#[derive(Serialize, Copy, Clone, EnumIter, Debug)]
#[allow(dead_code)]
pub enum Vertical {
    Finance,
    CS,
    Crypto,
    Philosophy,
    Business,
    ML,
    Enterme,
    Politics,
    Geography,
    Food,
    Math,
    Music,
    Economy,
    Activity,
    Society,
    Biology,
    Physics,
    Game,
    Medical,
    Transport,
    Law,
    Energy,
    Design,
    Language,
}

impl fmt::Display for Vertical {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Vertical::Finance => write!(f, "💰"),
            Vertical::CS => write!(f, "💻"),
            Vertical::Crypto => write!(f, "🥇"),
            Vertical::Philosophy => write!(f, "💎"),
            Vertical::Business => write!(f, "💼"),
            Vertical::ML => write!(f, "⚙️"),
            Vertical::Enterme => write!(f, "🎥"),
            Vertical::Politics => write!(f, "📢"),
            Vertical::Geography => write!(f, "🌏"),
            Vertical::Food => write!(f, "🍚"),
            Vertical::Math => write!(f, "📐"),
            Vertical::Music => write!(f, "🎵"),
            Vertical::Economy => write!(f, "💸"),
            Vertical::Activity => write!(f, "🥏"),
            Vertical::Society => write!(f, "🕸️"),
            Vertical::Biology => write!(f, "🦠"),
            Vertical::Physics => write!(f, "🍎"),
            Vertical::Game => write!(f, "🎮"),
            Vertical::Medical => write!(f, "💉"),
            Vertical::Transport => write!(f, "🚋"),
            Vertical::Law => write!(f, "⚖️"),
            Vertical::Energy => write!(f, "⚡"),
            Vertical::Design => write!(f, "🖼️"),
            Vertical::Language => write!(f, "🐨"),
        }
    }
}

#[derive(Serialize, Copy, Clone, EnumIter, Debug)]
#[allow(dead_code)]
pub enum Horizontal {
    Project,
    Content,
    Thought,
    Archive,
    Invisible,
}

#[derive(Serialize, Copy, Clone, EnumIter, Debug)]
#[allow(dead_code)]
pub enum External {
    Wikipedia,
    Investopedia,
    Amazon,
    Kindle,
}

#[derive(Serialize, Copy, Clone, EnumIter, Debug)]
#[allow(dead_code)]
pub enum Version {
    Aug2022,
    May2022,
}

#[derive(Serialize, Copy, Clone, EnumIter, Debug)]
#[allow(dead_code)]
pub enum Symbol {
    NEAR,
    Docker,
    NotionAutomation,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Symbol::NEAR => write!(f, "🔗"),
            Symbol::Docker => write!(f, "🐋"),
            Symbol::NotionAutomation => write!(f, "📦"),
        }
    }
}

#[derive(Serialize, Copy, Clone, EnumIter, Debug)]
#[allow(dead_code)]
pub enum Temporary {
    Debug,
    CannotRetrieve,
    NoVerticalPerson,
}
