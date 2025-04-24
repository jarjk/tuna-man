use super::players::Players;
use serde::{Deserialize, Serialize};
#[cfg(not(test))]
use std::io::Write;

/// a player/contestant/participant/team of a [`super::Tournament`]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default, Hash)]
pub struct Player {
    /// name of the Player
    pub name: String,
    /// seeding value of the Player
    pub seed: Option<u16>,
}
impl Player {
    pub fn new(name: impl AsRef<str>, seed: u16) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            seed: Some(seed),
        }
    }
    /// not yet initialized
    /// use in this case at your own risk
    pub fn is_unset(&self) -> bool {
        self == &Self::default()
    }
}
impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_unset() {
            write!(f, "{{waiting for player...}}")?;
            return Ok(());
        }
        let seed = if let Some(seed) = self.seed {
            [", ", &seed.to_string()].concat()
        } else {
            String::new()
        };
        write!(f, "{}{seed}", self.name,)
    }
}

/// A Duel/Match between two players.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Duel {
    pub homie: Player,
    pub guest: Player,
    /// homie won: true, opponent won: false
    pub outcome: Option<bool>,
}
impl std::fmt::Display for Duel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let outcome = self.outcome.map(|oc| {
            if oc {
                (&self.homie, &self.guest)
            } else {
                (&self.guest, &self.homie)
            }
        });
        if let Some((winner, loser)) = outcome {
            write!(f, "winner: {winner} <-> {loser} :loser")
        } else {
            write!(f, "{} <-> {}", self.homie, self.guest)
        }
    }
}
impl Duel {
    pub fn new(homie: Player, guest: Player) -> Self {
        Self {
            homie,
            guest,
            outcome: None,
        }
    }
    /// `self` but with `outcome`
    pub fn with_outcome(self, outcome: Option<bool>) -> Self {
        Self { outcome, ..self }
    }
    #[cfg(test)]
    pub fn get_outcome(&mut self) -> Result<(), ()> {
        self.outcome = Some(true);
        Ok(())
    }
    #[cfg(not(test))]
    pub fn get_outcome(&mut self) -> Result<(), ()> {
        print!("winner: ");
        std::io::stdout().flush().map_err(|_| ())?;
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).map_err(|_| ())?;
        let outcome = match buf.trim() {
            "<" | "homie" => Some(true),
            ">" | "guest" => Some(false),
            name => {
                let name = name.to_lowercase();
                if self.homie.name.to_lowercase().contains(&name) {
                    Some(true)
                } else if self.guest.name.to_lowercase().contains(&name) {
                    Some(false)
                } else {
                    // dbg!(&name);
                    if matches!(name.as_str(), "q" | "quit" | "exit") {
                        std::process::exit(0);
                    }
                    return Err(());
                }
            }
        };
        // println!("{self}");
        self.outcome = outcome;
        Ok(())
    }
    /// take winner of the game
    ///
    /// # Note
    ///
    /// it's taken: moved and replaced by [`Player::default()`]
    ///
    /// # Panics
    ///
    /// if there's no outcome yet
    fn take_winner(&mut self) -> Player {
        if self.outcome.unwrap() {
            std::mem::take(&mut self.homie)
        } else {
            std::mem::take(&mut self.guest)
        }
    }
    /// take loser of the game
    ///
    /// # Note
    ///
    /// it's taken: moved and replaced by [`Player::default()`]
    ///
    /// # Panics
    ///
    /// if there's no outcome yet
    fn take_loser(&mut self) -> Player {
        if self.outcome.unwrap() {
            std::mem::take(&mut self.guest)
        } else {
            std::mem::take(&mut self.homie)
        }
    }

    /// play the [`Duel`]: get an outcome with `read_outcome`
    /// **NOTE**: this moves [`Duel`]'s players
    pub fn play(mut self) -> (Player, Player) {
        loop {
            if let Ok(()) = self.get_outcome() {
                return (self.take_winner(), self.take_loser());
            }
            println!("invalid input");
        }
    }
    /// # Info
    ///
    /// - creates [`Duel`] from first two [`Player`]s of `branch`
    /// - plays the [`Duel`]
    /// - winner gets pushed back to the `branch`
    /// - loser gets returned
    ///
    /// # Warning
    ///
    /// there's a `println!()` hidden in here
    pub fn handle_special(branch: &mut Players) -> Player {
        let (homie, guest) = (branch.0.remove(0), branch.0.swap_remove(0)); // remove first two
        let duel = Duel::new(homie, guest); // create a duel
        println!("{duel}");
        let (winner, loser) = duel.play(); // play it
        branch.0.push(winner); // winner stays
        loser
    }
}
