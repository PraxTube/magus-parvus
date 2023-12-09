use super::Item;

pub fn item_title(item: &Item) -> String {
    let text = match item {
        Item::NotImplemented => "NOT IMPLEMENTED, you should not see this, please report",
        Item::Tutorial => "TUTORIAL: Spell Console",
        Item::IgnisPila => "UNLOCKED SPELL: Ignis Pila",
        Item::InfernoPila => "UNLOCKED SPELL: Inferno Pila",
        Item::Fulgur => "UNLOCKED SPELL: Fulgur",
    };
    text.to_string()
}

pub fn item_description(item: &Item) -> String {
    let text = match item {
        Item::NotImplemented => "CONTENT DESCRIPTION",
        Item::Tutorial => {
            "Press 'i' to open your spell console.\nThen type your spell, try 'fireball'."
        }
        Item::IgnisPila => "Cast 8 fireballs omni directionaly.",
        Item::InfernoPila => "Cast MANY fireballs omni directionaly",
        Item::Fulgur => {
            "Call down lightning strikes on random enemies.\nOnly works when there are enemies."
        }
    };
    text.to_string()
}
