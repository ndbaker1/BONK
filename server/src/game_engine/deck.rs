use crate::shared_types::{Card, CardName, CardRank, CardSuit};
use nanorand::{WyRand, RNG};

/// Creates a starting deck of Cards for the game
pub fn generate_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::with_capacity(80);
    // copying same cards atm
    for _ in 0..20 {
        deck.push(Card {
            name: CardName::Bang,
            suit: CardSuit::Clubs,
            rank: CardRank::N1,
        });
        deck.push(Card {
            name: CardName::Missed,
            suit: CardSuit::Hearts,
            rank: CardRank::N1,
        });
        deck.push(Card {
            name: CardName::Indians,
            suit: CardSuit::Diamonds,
            rank: CardRank::N1,
        });
        deck.push(Card {
            name: CardName::GeneralStore,
            suit: CardSuit::Diamonds,
            rank: CardRank::N1,
        });
        deck.push(Card {
            name: CardName::Duel,
            suit: CardSuit::Diamonds,
            rank: CardRank::N1,
        });
    }
    shuffle_deck(&mut deck);
    return deck;
}

/// Shuffles a vector of cards
pub fn shuffle_deck(deck: &mut Vec<Card>) {
    WyRand::new().shuffle(deck);
}
