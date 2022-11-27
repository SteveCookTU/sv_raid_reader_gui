#![allow(clippy::from_over_into)]

use sv_raid_reader::{Filter, GameProgress, GameVersion, Raid, ABILITIES, AREAS, GENDER_SYMBOLS, NATURES, SPECIES, TYPES, personal_table, PersonalInfo};

pub mod app;
mod draw;

const EVENT_FLATBUFFER_POINTER: [u64; 6] = [0x42DA820, 0x30, 0x388, 0x300, 0x28, 0x414];

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default)]
#[repr(u8)]
enum TeraType {
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy,
    #[default]
    Any,
}

impl TeraType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TeraType::Normal => "Normal",
            TeraType::Fighting => "Fighting",
            TeraType::Flying => "Flying",
            TeraType::Poison => "Poison",
            TeraType::Ground => "Ground",
            TeraType::Rock => "Rock",
            TeraType::Bug => "Bug",
            TeraType::Ghost => "Ghost",
            TeraType::Steel => "Steel",
            TeraType::Fire => "Fire",
            TeraType::Water => "Water",
            TeraType::Grass => "Grass",
            TeraType::Electric => "Electric",
            TeraType::Psychic => "Psychic",
            TeraType::Ice => "Ice",
            TeraType::Dragon => "Dragon",
            TeraType::Dark => "Dark",
            TeraType::Fairy => "Fairy",
            TeraType::Any => "Any",
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
enum Game {
    Scarlet,
    Violet,
}

impl Game {
    pub fn as_str(&self) -> &'static str {
        match self {
            Game::Scarlet => "Scarlet",
            Game::Violet => "Violet",
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
enum Progress {
    None,
    Badge3,
    Badge7,
    Credits,
    PostGame,
}

impl Progress {
    pub fn as_str(&self) -> &'static str {
        match self {
            Progress::None => "None",
            Progress::Badge3 => "Badge 3",
            Progress::Badge7 => "Badge 7",
            Progress::Credits => "Credits",
            Progress::PostGame => "Post Game",
        }
    }
}

impl Into<GameVersion> for Game {
    fn into(self) -> GameVersion {
        match self {
            Game::Scarlet => GameVersion::Scarlet,
            Game::Violet => GameVersion::Violet,
        }
    }
}

impl Into<GameProgress> for Progress {
    fn into(self) -> GameProgress {
        match self {
            Progress::None => GameProgress::None,
            Progress::Badge3 => GameProgress::Badge3,
            Progress::Badge7 => GameProgress::Badge7,
            Progress::Credits => GameProgress::Credits,
            Progress::PostGame => GameProgress::PostGame,
        }
    }
}

#[derive(Copy, Clone, Default)]
struct FilterWrapper {
    pub tera_type: TeraType,
    pub species: u16,
    pub star_level: u8,
    pub shiny: bool,
    pub event: bool,
}

impl Into<Filter> for FilterWrapper {
    fn into(self) -> Filter {
        Filter {
            tera_type: if self.tera_type == TeraType::Any {
                None
            } else {
                Some(self.tera_type as u8)
            },
            star_level: if self.star_level == 0 {
                None
            } else {
                Some(self.star_level)
            },
            shiny: self.shiny,
            event: self.event,
            species: if self.species == 0 {
                None
            } else {
                Some(self.species)
            },
        }
    }
}

struct RaidResult {
    area: &'static str,
    seed: String,
    event: &'static str,
    shiny: &'static str,
    species: &'static str,
    gender: String,
    pid: String,
    hp: String,
    atk: String,
    def: String,
    spa: String,
    spd: String,
    spe: String,
    ability: &'static str,
    nature: &'static str,
    star_level: String,
    tera: &'static str,
}

impl Into<RaidResult> for Raid {
    fn into(self) -> RaidResult {

        let personal = personal_table::SV.get_form_entry(self.species as usize, self.form as usize);
        let ability = personal
            .get_ability_index((self.pokemon.ability - 1) as usize)
            .unwrap();

        RaidResult {
            area: AREAS[self.area as usize - 1],
            seed: format!("{:0>8X}", self.seed),
            event: if self.event { "O" } else { "X" },
            shiny: if self.pokemon.shiny { "!!!" } else { "" },
            species: SPECIES[self.species as usize],
            gender: GENDER_SYMBOLS[self.pokemon.gender as usize].to_string(),
            pid: format!("{:0>8X}", self.pokemon.pid),
            hp: format!("{:0>2}", self.pokemon.ivs[0]),
            atk: format!("{:0>2}", self.pokemon.ivs[1]),
            def: format!("{:0>2}", self.pokemon.ivs[2]),
            spa: format!("{:0>2}", self.pokemon.ivs[3]),
            spd: format!("{:0>2}", self.pokemon.ivs[4]),
            spe: format!("{:0>2}", self.pokemon.ivs[5]),
            ability: ABILITIES[ability],
            nature: NATURES[self.pokemon.nature as usize],
            star_level: format!("{}", self.star_level),
            tera: TYPES[self.tera_type as usize],
        }
    }
}
