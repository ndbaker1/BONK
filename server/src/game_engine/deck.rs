use crate::shared_types;
use nanorand::{WyRand, RNG};

/// Creates a starting deck of Cards for the game
pub fn generate_deck() -> Vec<shared_types::Card> {
    let mut deck: Vec<shared_types::Card> = Vec::with_capacity(80);
    // copying same cards atm
    for _ in 0..20 {
        deck.push(shared_types::Card {
            name: shared_types::CardName::Bang,
            suit: shared_types::CardSuit::Clubs,
            rank: shared_types::CardRank::N1,
        });
        deck.push(shared_types::Card {
            name: shared_types::CardName::Missed,
            suit: shared_types::CardSuit::Hearts,
            rank: shared_types::CardRank::N1,
        });
        // deck.push(shared_types::Card {
        //     name: shared_types::CardName::Indians,
        //     suit: shared_types::CardSuit::Diamonds,
        //     rank: shared_types::CardRank::N1,
        // });
    }
    shuffle_deck(&mut deck);
    return deck;
}

/// Shuffles a vector of cards
pub fn shuffle_deck(deck: &mut Vec<shared_types::Card>) {
    WyRand::new().shuffle(deck);
}
