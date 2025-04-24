use super::structs::*;
use std::path::Path;

#[cfg(test)]
pub mod tests;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Players(pub Vec<Player>);

impl Players {
    /// load players from file at `path`
    pub fn load(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let mut reader = csv::Reader::from_path(path)?;
        let players = reader
            .deserialize()
            .flat_map(|x| x.inspect_err(|e| eprintln!("error: {e:#?}")))
            .collect();
        Ok(Self(players))
    }
    /// save `self` to file at `path`
    pub fn save(self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let mut writer = csv::Writer::from_path(path)?;
        self.0.iter().try_for_each(|p| writer.serialize(p))?;
        writer.flush()
    }
    /// seed if can, `shuffle` otherwise, and order, so that every two following players make up a [`Duel`]
    pub fn seed(&mut self) {
        if self.0.first().is_some_and(|p| p.seed.is_none()) {
            // if no seed present: shuffle to make match-making unpredictable
            fastrand::shuffle(&mut self.0);
            return;
        }
        // here'll be the players ordered as pairs
        let mut as_pairs = Vec::new();
        self.0.sort_unstable_by_key(|p| p.seed);
        // 2 players always needed to make up a duel
        while self.0.len() > 1 {
            as_pairs.push(self.0.remove(0)); // first the current player
            as_pairs.push(self.0.pop().unwrap()); // then the last one
        }
        // someone's remained, it's pushed to end
        as_pairs.append(&mut self.0);
        self.0 = as_pairs; // apply changes
    }
    /// convert `self` into [`Duel`]s
    pub fn into_duels(mut self, seed: bool) -> Vec<Duel> {
        if seed {
            // seed, fallback to shuffle and sort into pairs
            self.seed();
        }

        // if needs bye: add one
        if self.0.len() % 2 == 1 {
            self.0.push(Player::default());
        }

        self.0
            .rchunks_exact_mut(2)
            .map(|c| Duel::new(std::mem::take(&mut c[0]), std::mem::take(&mut c[1])))
            .collect()
    }
}
