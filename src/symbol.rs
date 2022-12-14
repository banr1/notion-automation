use std::fmt;
use strum::EnumIter;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, EnumIter, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum Symbol {
    // Vertical
    Finance,
    Computer,
    Crypto,
    Philosophy,
    Business,
    Stat,
    Enterme,
    Politics,
    Geography,
    Food,
    Math,
    Music,
    Activity,
    Society,
    Biology,
    Physics,
    Game,
    Medicine,
    Transport,
    Law,
    Energy,
    Design,
    Language,

    BusinessFinance,
    CryptoBusiness,
    CryptoComputer,
    CryptoFinance,
    CryptoFinanceStat,
    CryptoGame,
    CryptoLaw,
    FinanceComputer,
    FinanceEnterme,
    FinanceStat,
    GeographyFinance,
    PoliticsBusiness,
    PoliticsFinance,
    StatComputer,

    // Horizontal
    Project,

    // Product (Crypto)
    Arweave,
    Astar,
    Avalanche,
    Axie,
    Binance,
    Bitcoin,
    BitcoinFinance,
    Bitfinex,
    Bybit,
    Celo,
    Chainlink,
    Cosmos,
    Ethereum,
    Fantom,
    Flow,
    #[serde(rename = "FTX")]
    Ftx,
    GodsUnchained,
    Harmony,
    InternetComputer,
    Move,
    #[serde(rename = "NEAR")]
    Near,
    Polkadot,
    Polygon,
    Solana,
    Solidity,
    #[serde(rename = "STEPN")]
    Stepn,
    ZeroToHero,
    // Product (Computer)
    Apple,
    #[serde(rename = "AWS")]
    Aws,
    Azure,
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
    Go,
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
    #[serde(rename = "SQL")]
    Sql,
    Swift,
    TypeScript,
    #[serde(rename = "VSCode")]
    VsCode,
    // Product (Other)
    Antifragile,
    DuelMasters,
    EnglishPhrase,
    EnglishWord,
    Figma,
    Forex,
    Human,
    JapaneseLang,
    LawOfJapan,
    #[serde(rename = "MHRise")]
    MhRise,
    Notion,
    Pandas,
    TradingView,
    Stock,

    // Project
    Avilen,
    BusinessIdea,
    #[serde(rename = "DeNA")]
    Dena,
    Drivearth,
    Friend,
    Gemma,
    JQuants,
    #[serde(rename = "MUTB")]
    Mutb,
    NotionAutomation,
    QuaternityBot,
    #[serde(rename = "RMS")]
    Rms,
    SelfUnderstanding,
    #[serde(rename = "SMFGCompe")]
    SmfgCompe,
    #[serde(rename = "TA")]
    Ta,
    Trading,
    Trajectory,
    TrivialNotes,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Symbol::Finance => write!(f, "????"),
            Symbol::Computer => write!(f, "????"),
            Symbol::Crypto => write!(f, "????"),
            Symbol::Philosophy => write!(f, "????"),
            Symbol::Business => write!(f, "????"),
            Symbol::Stat => write!(f, "????"),
            Symbol::Enterme => write!(f, "????"),
            Symbol::Politics => write!(f, "????"),
            Symbol::Geography => write!(f, "????"),
            Symbol::Food => write!(f, "????"),
            Symbol::Math => write!(f, "????"),
            Symbol::Music => write!(f, "????"),
            Symbol::Activity => write!(f, "????"),
            Symbol::Society => write!(f, "???????"),
            Symbol::Biology => write!(f, "????"),
            Symbol::Physics => write!(f, "????"),
            Symbol::Game => write!(f, "????"),
            Symbol::Medicine => write!(f, "????"),
            Symbol::Transport => write!(f, "????"),
            Symbol::Law => write!(f, "??????"),
            Symbol::Energy => write!(f, "???"),
            Symbol::Design => write!(f, "???????"),
            Symbol::Language => write!(f, "????"),

            Symbol::Project => write!(f, "????"),

            Symbol::BusinessFinance => write!(f, "????"),
            Symbol::CryptoBusiness => write!(f, "????"),
            Symbol::CryptoComputer => write!(f, "????"),
            Symbol::CryptoFinance => write!(f, "??????"),
            Symbol::CryptoFinanceStat => write!(f, "??????"),
            Symbol::CryptoGame => write!(f, "????"),
            Symbol::CryptoLaw => write!(f, "????"),
            Symbol::FinanceComputer => write!(f, "????"),
            Symbol::FinanceEnterme => write!(f, "???????"),
            Symbol::FinanceStat => write!(f, "????"),
            Symbol::GeographyFinance => write!(f, "????"),
            Symbol::PoliticsBusiness => write!(f, "????"),
            Symbol::PoliticsFinance => write!(f, "????"),
            Symbol::StatComputer => write!(f, "????"),

            Symbol::Arweave => write!(f, "????"),
            Symbol::Astar => write!(f, "????"),
            Symbol::Avalanche => write!(f, "????"),
            Symbol::Axie => write!(f, "????"),
            Symbol::Binance => write!(f, "????"),
            Symbol::Bitcoin => write!(f, "???????"),
            Symbol::BitcoinFinance => write!(f, "????"),
            Symbol::Bitfinex => write!(f, "????"),
            Symbol::Bybit => write!(f, "????"),
            Symbol::Celo => write!(f, "????"),
            Symbol::Chainlink => write!(f, "????"),
            Symbol::Cosmos => write!(f, "????"),
            Symbol::Ethereum => write!(f, "????"),
            Symbol::Fantom => write!(f, "????"),
            Symbol::Flow => write!(f, "????"),
            Symbol::Ftx => write!(f, "???????"),
            Symbol::GodsUnchained => write!(f, "????"),
            Symbol::Harmony => write!(f, "????"),
            Symbol::InternetComputer => write!(f, "????"),
            Symbol::Move => write!(f, "????"),
            Symbol::Near => write!(f, "????"),
            Symbol::Polkadot => write!(f, "????"),
            Symbol::Polygon => write!(f, "????"),
            Symbol::Solana => write!(f, "??????"),
            Symbol::Solidity => write!(f, "??????"),
            Symbol::Stepn => write!(f, "????"),
            Symbol::ZeroToHero => write!(f, "????????"),

            Symbol::Apple => write!(f, "???????"),
            Symbol::Aws => write!(f, "????"),
            Symbol::Azure => write!(f, "????"),
            Symbol::C => write!(f, "????"),
            Symbol::CPlusPlus => write!(f, "?????????????"),
            Symbol::CSharp => write!(f, "?????????????"),
            Symbol::Css => write!(f, "????"),
            Symbol::Docker => write!(f, "????"),
            Symbol::Flutter => write!(f, "????"),
            Symbol::Gcp => write!(f, "???"),
            Symbol::Git => write!(f, "????"),
            Symbol::GitHub => write!(f, "????"),
            Symbol::GitLab => write!(f, "????"),
            Symbol::Go => write!(f, "????"),
            Symbol::Haskell => write!(f, "????"),
            Symbol::Html => write!(f, "????"),
            Symbol::Java => write!(f, "????"),
            Symbol::JavaScript => write!(f, "????"),
            Symbol::Kotlin => write!(f, "????"),
            Symbol::Linux => write!(f, "????"),
            Symbol::Mantine => write!(f, "????"),
            Symbol::Python => write!(f, "????"),
            Symbol::React => write!(f, "????"),
            Symbol::Ruby => write!(f, "????"),
            Symbol::Rust => write!(f, "??????"),
            Symbol::Scala => write!(f, "????"),
            Symbol::Sql => write!(f, "????"),
            Symbol::Swift => write!(f, "????"),
            Symbol::TypeScript => write!(f, "????"),
            Symbol::VsCode => write!(f, "????"),

            Symbol::Antifragile => write!(f, "???????"),
            Symbol::DuelMasters => write!(f, "????"),
            Symbol::EnglishPhrase => write!(f, "??????"),
            Symbol::EnglishWord => write!(f, "????"),
            Symbol::Figma => write!(f, "????"),
            Symbol::Forex => write!(f, "???????"),
            Symbol::Human => write!(f, "?????????????"),
            Symbol::JapaneseLang => write!(f, "????"),
            Symbol::LawOfJapan => write!(f, "????????"),
            Symbol::MhRise => write!(f, "????"),
            Symbol::Notion => write!(f, "????"),
            Symbol::Pandas => write!(f, "????"),
            Symbol::TradingView => write!(f, "????"),
            Symbol::Stock => write!(f, "????"),

            Symbol::Avilen => write!(f, "????"),
            Symbol::BusinessIdea => write!(f, "????"),
            Symbol::Dena => write!(f, "?????????????"),
            Symbol::Drivearth => write!(f, "????"),
            Symbol::Friend => write!(f, "????"),
            Symbol::Gemma => write!(f, "??????"),
            Symbol::JQuants => write!(f, "????"),
            Symbol::Mutb => write!(f, "???"),
            Symbol::NotionAutomation => write!(f, "????"),
            Symbol::QuaternityBot => write!(f, "????"),
            Symbol::Rms => write!(f, "???"),
            Symbol::SelfUnderstanding => write!(f, "????"),
            Symbol::SmfgCompe => write!(f, "????"),
            Symbol::Ta => write!(f, "????"),
            Symbol::Trading => write!(f, "????"),
            Symbol::Trajectory => write!(f, "???????"),
            Symbol::TrivialNotes => write!(f, "???"),
            // Symbol::XXX => write!(f, "X"),
            // Symbol::XXX => write!(f, "X"),
            // Symbol::XXX => write!(f, "X"),
            // Symbol::XXX => write!(f, "X"),
            // Symbol::XXX => write!(f, "X"),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct SymbolProperty {
    pub id: String,
    pub select: SymbolSelect,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SymbolSelect {
    pub name: Symbol,
}
