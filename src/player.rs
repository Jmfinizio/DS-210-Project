use serde_derive::Deserialize;
use strsim::levenshtein;

pub trait Attributes {
    fn nation(&self) -> String;
    fn team(&self) -> String;
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Player {
    pub name: String,
    pub rating: u8,
    pub position: String,
    pub nation: String,
    pub team: String,
    pub pac: u8,
    pub sho: u8,
    pub pas: u8,
    pub dri: u8,
    pub def: u8,
    pub phy: u8,
}

impl Attributes for Player {
    fn nation(&self) -> String {
        (*self.nation).to_string()
    }
    
    fn team(&self) -> String {
        (*self.team).to_string()
    } 
}


impl Player {
    fn create(name: String, rating:u8, position: String, nation: String, team: String, pac: u8,
    sho: u8, pas: u8, dri: u8, def: u8, phy: u8) -> Player {
        Player{name,rating,position,nation,team,pac,sho,pas,dri,def,phy}
    }
}

pub fn find_player<'a>(players: &'a [Player], input_name: &'a str) -> Option<&'a Player> {
    if let Some(player) = players.iter().find(|p| p.name == input_name) {
        return Some(player);
    }
   
    let mut closest_distance = usize::MAX;
    let mut closest_match: Option<&Player> = None;

    for player in players {
        let distance = levenshtein(player.name.as_str(), input_name);

        if distance < closest_distance {
            closest_distance = distance;
            closest_match = Some(player);
        }
    }

    if closest_distance <= 3 {
        println!("Did you mean {}?", closest_match.unwrap().name);
    }

    None
}