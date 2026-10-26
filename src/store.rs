//! Players and their stars, kept in one small JSON file.

use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
};

use crate::levels;

#[derive(Serialize, Deserialize, Clone)]
pub struct Profile {
    pub id: u64,
    pub name: String,
    pub avatar: String,
    /// Best stars (1–3) per level id.
    #[serde(default)]
    pub stars: BTreeMap<String, u8>,
    /// Biggest run (50 or 100) finished with 3 stars, per level id.
    #[serde(default)]
    pub medals: BTreeMap<String, u32>,
    /// Levels whose "Show me how" lesson this kid has done (or skipped).
    #[serde(default)]
    pub learned: BTreeSet<String>,
    #[serde(default)]
    pub solved: u32,
    #[serde(default)]
    pub best_streak: u32,
    /// For older kids who want to jump ahead, set by a grown-up.
    #[serde(default)]
    pub unlock_all: bool,
}

impl Profile {
    pub fn stars(&self, level: &str) -> u8 {
        self.stars.get(level).copied().unwrap_or(0)
    }

    pub fn medal(&self, level: &str) -> u32 {
        self.medals.get(level).copied().unwrap_or(0)
    }

    /// Kiko's pick for how many questions, from how this kid did on the level.
    pub fn suggested_count(&self, level: &str) -> (usize, &'static str) {
        match (self.stars(level), self.medal(level)) {
            (_, 50..) => (100, "You have the Challenge medal here. Ready for a Marathon?"),
            (3, _) => (50, "You got 3 stars here, so let's try a Challenge!"),
            (1 | 2, _) => (20, "Some practice will get you to 3 stars."),
            _ => (10, "This one is new, so let's start small."),
        }
    }

    pub fn total_stars(&self) -> u32 {
        self.stars.values().map(|&s| s as u32).sum()
    }

    /// Levels open one after another: finishing a level opens the next.
    pub fn unlocked(&self, flat: usize) -> bool {
        self.unlock_all || flat == 0 || levels::all().nth(flat - 1).is_some_and(|(_, l)| self.stars(l.id) > 0)
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Store {
    pub profiles: Vec<Profile>,
    #[serde(default = "yes")]
    pub sound: bool,
    #[serde(skip)]
    path: PathBuf,
}

fn yes() -> bool {
    true
}

impl Store {
    /// $BILANG_DATA overrides the folder, for testing.
    pub fn load() -> Self {
        let dir = std::env::var_os("BILANG_DATA").map(PathBuf::from).unwrap_or_else(|| {
            directories::ProjectDirs::from("io", "r3al", "Bilang").map(|d| d.data_dir().to_path_buf()).unwrap_or_else(|| PathBuf::from("."))
        });
        let path = dir.join("players.json");
        let mut store: Store = std::fs::read(&path)
            .ok()
            .and_then(|b| serde_json::from_slice(&b).ok())
            .unwrap_or(Store { sound: true, ..Default::default() });
        store.path = path;
        store
    }

    pub fn save(&self) {
        if let Some(dir) = self.path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let tmp = self.path.with_extension("json.tmp");
        match serde_json::to_vec_pretty(self) {
            Ok(b) if std::fs::write(&tmp, &b).is_ok() => {
                let _ = std::fs::rename(&tmp, &self.path);
            }
            _ => eprintln!("bilang: could not save {}", self.path.display()),
        }
    }

    pub fn add(&mut self, name: String, avatar: String) -> usize {
        let id = self.profiles.iter().map(|p| p.id).max().unwrap_or(0) + 1;
        self.profiles.push(Profile { id, name, avatar, stars: BTreeMap::new(), medals: BTreeMap::new(), learned: BTreeSet::new(), solved: 0, best_streak: 0, unlock_all: false });
        self.save();
        self.profiles.len() - 1
    }
}
