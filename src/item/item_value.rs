use super::Item;

pub fn item_title(item: &Item) -> String {
    let text = match item {
        Item::Test => "TEST, you should not see this, please report",
        Item::Fulgur => "UNLOCKED SPELL: Fulgur",
    };
    text.to_string()
}

pub fn item_description(item: &Item) -> String {
    let text = match item {
        Item::Test => "CONTENT DESCRIPTION",
        Item::Fulgur => {
            "Call down lightning strikes on random enemies.\nOnly works when there are enemies."
        }
    };
    text.to_string()
}
