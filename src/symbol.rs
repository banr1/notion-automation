use std::fmt;
use strum::EnumIter;

use serde::Serialize;

#[derive(Serialize, Copy, Clone, EnumIter, Debug)]
#[allow(dead_code)]
pub enum Symbol {
    // Vertical
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

    // Product (Crypto)
    Astar,
    Avalanche,
    Binance,
    Bitcoin,
    Docker,
    Ethereum,
    InternetComputer,
    NEAR,
    Polkadot,
    Solana,
    STEPN,
    // Product (CS)
    AWS,
    CSS,
    GCP,
    Git,
    Github,
    Gitlab,
    HTML,
    Python,
    React,
    Rust,
    Typescript,
    // Product (Other)
    Antifragile,
    Notion,
    Pandas,

    // Project
    Avilen,
    Drivearth,
    Gemma,
    NotionAutomation,
    QuaternityBot,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Symbol::Finance => write!(f, "💰"),
            Symbol::CS => write!(f, "💻"),
            Symbol::Crypto => write!(f, "🥇"),
            Symbol::Philosophy => write!(f, "💎"),
            Symbol::Business => write!(f, "💼"),
            Symbol::ML => write!(f, "🎲"),
            Symbol::Enterme => write!(f, "🎥"),
            Symbol::Politics => write!(f, "📢"),
            Symbol::Geography => write!(f, "🌏"),
            Symbol::Food => write!(f, "🍚"),
            Symbol::Math => write!(f, "📐"),
            Symbol::Music => write!(f, "🎵"),
            Symbol::Economy => write!(f, "💸"),
            Symbol::Activity => write!(f, "🥏"),
            Symbol::Society => write!(f, "🕸️"),
            Symbol::Biology => write!(f, "🦠"),
            Symbol::Physics => write!(f, "🍎"),
            Symbol::Game => write!(f, "🎮"),
            Symbol::Medical => write!(f, "💉"),
            Symbol::Transport => write!(f, "🚋"),
            Symbol::Law => write!(f, "⚖️"),
            Symbol::Energy => write!(f, "⚡"),
            Symbol::Design => write!(f, "🖼️"),
            Symbol::Language => write!(f, "🐨"),

            Symbol::Astar => write!(f, "🍬"),
            Symbol::Avalanche => write!(f, "🔺"),
            Symbol::Binance => write!(f, "🔶"),
            Symbol::Bitcoin => write!(f, "🟠"),
            Symbol::Docker => write!(f, "🐋"),
            Symbol::Ethereum => write!(f, "⚕️"),
            Symbol::InternetComputer => write!(f, "🪢"),
            Symbol::NEAR => write!(f, "🔗"),
            Symbol::Polkadot => write!(f, "👚"),
            Symbol::Solana => write!(f, "🐠"),
            Symbol::STEPN => write!(f, "🥗"),

            Symbol::AWS => write!(f, "🥕"),
            Symbol::CSS => write!(f, "👖"),
            Symbol::Git => write!(f, "♦️"),
            Symbol::Github => write!(f, "🐱"),
            Symbol::Gitlab => write!(f, "🦊"),
            Symbol::GCP => write!(f, "⛅"),
            Symbol::HTML => write!(f, "🧀"),
            Symbol::Python => write!(f, "🐍"),
            Symbol::React => write!(f, "🫐"),
            Symbol::Rust => write!(f, "⚙️"),
            Symbol::Typescript => write!(f, "🟦"),

            Symbol::Antifragile => write!(f, "🅰️"),
            Symbol::Notion => write!(f, "🔲"),
            Symbol::Pandas => write!(f, "🐼"),

            Symbol::Avilen => write!(f, "🐵"),
            Symbol::Drivearth => write!(f, "🚘"),
            Symbol::Gemma => write!(f, "♠️"),
            Symbol::NotionAutomation => write!(f, "📦"),
            Symbol::QuaternityBot => write!(f, "🤖"),
        }
    }
}
