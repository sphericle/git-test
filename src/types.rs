use crate::rocket::serde;

#[derive(Debug, serde::Serialize)]
pub enum Difficulty {
    ImpossibleLayout,
    SilentLayout,
    LegendaryLayout,
    EtherealLayout,
    SupremeLayout,
    ExtremeLayout,
    MythicalLayout,
    InsaneLayout,
    HardLayout,
    MediumLayout,
    EasyLayout,
    BeginnerLayout,
    None,
}

#[derive(serde::Serialize)]
pub struct Level {
    pub id: i64,
    pub name: String,
    pub creators: Vec<String>,
    pub verifier: String,
    pub verification: String,
    pub percent_to_qualify: f64,
    pub song_name: String,
    pub song_link: Option<String>,
    pub difficulty: Difficulty,
    pub records: Vec<Record>,
}

#[derive(serde::Serialize)]
pub struct Record {
    pub user: String,
    pub link: String,
    pub percent: i8,
    pub hz: i16,
    pub mobile: bool,
    pub enjoyment: Option<i8>,
}
