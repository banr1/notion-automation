use std::fmt;
use strum::EnumIter;

use serde::Serialize;

#[derive(Serialize, Copy, Clone, EnumIter, Debug)]
#[allow(dead_code)]
pub enum Symbol {
    // Vertical
    Finance,
    #[serde(rename = "CS")]
    Cs,
    Crypto,
    Philosophy,
    Business,
    #[serde(rename = "ML")]
    Ml,
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

    BusinessFinance,
    CryptoCS,
    CryptoEconomy,
    CryptoFinance,
    CryptoGame,
    CryptoLaw,

    // Product (Crypto)
    Astar,
    Avalanche,
    Binance,
    Bitcoin,
    Celo,
    Cosmos,
    Ethereum,
    Flow,
    InternetComputer,
    #[serde(rename = "NEAR")]
    Near,
    Polkadot,
    Polygon,
    Solana,
    Solidity,
    #[serde(rename = "STEPN")]
    Stepn,
    ZeroToHero,
    // Product (CS)
    Apple,
    #[serde(rename = "AWS")]
    Aws,
    C,
    #[serde(rename = "C++")]
    CPlusPlus,
    #[serde(rename = "C#")]
    CSharp,
    #[serde(rename = "CSS")]
    Css,
    Docker,
    Flutter,
    #[serde(rename = "GCP")]
    Gcp,
    Git,
    GitHub,
    GitLab,
    Haskell,
    #[serde(rename = "HTML")]
    Html,
    Java,
    JavaScript,
    Kotlin,
    Linux,
    Mantine,
    Python,
    React,
    Ruby,
    Rust,
    Scala,
    Swift,
    TypeScript,
    // Product (Other)
    Antifragile,
    #[serde(rename = "MHRise")]
    MhRise,
    Notion,
    Pandas,

    // Project
    Avilen,
    #[serde(rename = "DeNA")]
    Dena,
    Drivearth,
    Gemma,
    #[serde(rename = "MUTB")]
    Mutb,
    NotionAutomation,
    QuaternityBot,
    #[serde(rename = "SMFGCompe")]
    SmfgCompe,
    Trajectory,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Symbol::Finance => write!(f, "💰"),
            Symbol::Cs => write!(f, "💻"),
            Symbol::Crypto => write!(f, "🥇"),
            Symbol::Philosophy => write!(f, "💎"),
            Symbol::Business => write!(f, "💼"),
            Symbol::Ml => write!(f, "🎲"),
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

            Symbol::BusinessFinance => write!(f, "🧾"),
            Symbol::CryptoCS => write!(f, "🍿"),
            Symbol::CryptoEconomy => write!(f, "🪶"),
            Symbol::CryptoFinance => write!(f, "〽️"),
            Symbol::CryptoGame => write!(f, "🥌"),
            Symbol::CryptoLaw => write!(f, "🔨"),

            Symbol::Astar => write!(f, "🍬"),
            Symbol::Avalanche => write!(f, "🔺"),
            Symbol::Binance => write!(f, "🍯"),
            Symbol::Bitcoin => write!(f, "🏵️"),
            Symbol::Celo => write!(f, "🪲"),
            Symbol::Cosmos => write!(f, "🌑"),
            Symbol::Ethereum => write!(f, "🕋"),
            Symbol::Flow => write!(f, "🎾"),
            Symbol::InternetComputer => write!(f, "🪢"),
            Symbol::Near => write!(f, "🔗"),
            Symbol::Polkadot => write!(f, "👚"),
            Symbol::Polygon => write!(f, "👾"),
            Symbol::Solana => write!(f, "🐠"),
            Symbol::Solidity => write!(f, "♟️"),
            Symbol::Stepn => write!(f, "🥗"),
            Symbol::ZeroToHero => write!(f, "🦸🏽"),

            Symbol::Apple => write!(f, "🖥️"),
            Symbol::Aws => write!(f, "🧀"),
            Symbol::C => write!(f, "🧞"),
            Symbol::CPlusPlus => write!(f, "🧞‍♂️"),
            Symbol::CSharp => write!(f, "🧞‍♀️"),
            Symbol::Css => write!(f, "🔹"),
            Symbol::Docker => write!(f, "🐋"),
            Symbol::Flutter => write!(f, "🦕"),
            Symbol::Gcp => write!(f, "⛅"),
            Symbol::Git => write!(f, "🏮"),
            Symbol::GitHub => write!(f, "🐱"),
            Symbol::GitLab => write!(f, "🦊"),
            Symbol::Haskell => write!(f, "🦿"),
            Symbol::Html => write!(f, "🔸"),
            Symbol::Java => write!(f, "💈"),
            Symbol::JavaScript => write!(f, "🌮"),
            Symbol::Kotlin => write!(f, "🚎"),
            Symbol::Linux => write!(f, "🐧"),
            Symbol::Mantine => write!(f, "🥣"),
            Symbol::Python => write!(f, "🐍"),
            Symbol::React => write!(f, "🫐"),
            Symbol::Ruby => write!(f, "🐞"),
            Symbol::Rust => write!(f, "⚙️"),
            Symbol::Scala => write!(f, "💄"),
            Symbol::Swift => write!(f, "🦃"),
            Symbol::TypeScript => write!(f, "🧊"),

            Symbol::Antifragile => write!(f, "🅰️"),
            Symbol::MhRise => write!(f, "🦖"),
            Symbol::Notion => write!(f, "🔲"),
            Symbol::Pandas => write!(f, "🐼"),

            Symbol::Avilen => write!(f, "🐵"),
            Symbol::Dena => write!(f, "🐻‍❄️"),
            Symbol::Drivearth => write!(f, "🚘"),
            Symbol::Gemma => write!(f, "♠️"),
            Symbol::Mutb => write!(f, "⛽"),
            Symbol::NotionAutomation => write!(f, "📦"),
            Symbol::SmfgCompe => write!(f, "🥒"),
            Symbol::QuaternityBot => write!(f, "🤖"),
            Symbol::Trajectory => write!(f, "🕰️"),
        }
    }
}
