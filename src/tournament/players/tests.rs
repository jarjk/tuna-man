use super::*;
use pretty_assertions::assert_eq;

pub fn load_players() -> Players {
    Players::load("data.csv").unwrap()
}

#[test]
#[should_panic]
fn antiload() {
    Players::load("Low for the sake of environment.bacilus").unwrap();
}

pub fn nu_p(name: &str, seed: u16) -> Player {
    Player::new(name, seed)
}

#[test]
fn load() {
    assert_eq!(
        Players(vec![
            nu_p("Central Mite", 2),
            nu_p("Relative Wrasse", 2),
            nu_p("Exotic Skunk", 0),
            nu_p("Droll Jaguar", 4),
            nu_p("Usable Bengal", 1),
            nu_p("Inviting Pheasant", 4),
            nu_p("Profound Ponytail", 0),
            nu_p("Expectant Wolfhound", 1),
            nu_p("Casual Ptarmigan", 3)
        ]),
        load_players()
    );
}

#[test]
fn seeding_players() {
    let mut players = load_players();
    players.seed();
    assert_eq!(
        players,
        Players(vec![
            nu_p("Exotic Skunk", 0),
            nu_p("Inviting Pheasant", 4),
            nu_p("Profound Ponytail", 0),
            nu_p("Droll Jaguar", 4),
            nu_p("Usable Bengal", 1),
            nu_p("Casual Ptarmigan", 3),
            nu_p("Expectant Wolfhound", 1),
            nu_p("Relative Wrasse", 2),
            nu_p("Central Mite", 2),
        ])
    );
}

#[test]
fn seeding_basic() {
    let mut players = Players(vec![
        nu_p("a", 1),
        nu_p("b", 2),
        nu_p("c", 3),
        nu_p("d", 4),
        nu_p("e", 5),
        nu_p("f", 6),
        nu_p("g", 7),
        nu_p("h", 8),
    ]);
    players.seed();
    assert_eq!(
        players,
        Players(vec![
            nu_p("a", 1),
            nu_p("h", 8),
            nu_p("b", 2),
            nu_p("g", 7),
            nu_p("c", 3),
            nu_p("f", 6),
            nu_p("d", 4),
            nu_p("e", 5),
        ])
    );
}
