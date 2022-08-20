use serde::Serialize;

#[derive(Serialize)]
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

#[derive(Serialize)]
#[allow(dead_code)]
pub enum Horizontal {
    Project,
    Content,
    Thought,
    Archive,
    Invisible,
}

#[derive(Serialize)]
#[allow(dead_code)]
pub enum External {
    Wikipedia,
    Investopedia,
}

#[derive(Serialize)]
#[allow(dead_code)]
pub enum Version {
    Aug2022,
    May2022,
    Mar2022,
    Feb2022,
    Jan2022,
}

#[derive(Serialize)]
#[serde(untagged)]
#[allow(dead_code)]
pub enum MultiSelectColumn {
    Vertical(Vertical),
    Horizontal(Horizontal),
    External(External),
    Version(Version),
}
