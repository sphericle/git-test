use crate::rocket::serde;

#[derive(Debug, serde::Serialize, PartialEq)]
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

impl Difficulty {
    pub fn map_index(index: Option<u64>) -> Difficulty {
        match index {
            Some(0) => Difficulty::BeginnerLayout,
            Some(1) => Difficulty::EasyLayout,
            Some(2) => Difficulty::MediumLayout,
            Some(3) => Difficulty::HardLayout,
            Some(4) => Difficulty::InsaneLayout,
            Some(5) => Difficulty::MythicalLayout,
            Some(6) => Difficulty::ExtremeLayout,
            Some(7) => Difficulty::SupremeLayout,
            Some(8) => Difficulty::EtherealLayout,
            Some(9) => Difficulty::LegendaryLayout,
            Some(10) => Difficulty::SilentLayout,
            Some(11) => Difficulty::ImpossibleLayout,
            Some(12..) | None => Difficulty::None,
        }
    }
}
