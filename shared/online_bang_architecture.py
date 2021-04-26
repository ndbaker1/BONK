
Code Architecture
-------------------------------------

'''
    Events that can happen throughout the gameplay,
    which character abilities can trigger on
'''
EVENTS {
    draw
    hand-empty
    discarded
    targetted-by-bang
    targetted-by-damage
    lost-hp
    draw!

}


'''
    Cards will have an ID
'''
struct CARD {
    id: INT
    name: STRING
    type: 'GREEN' | 'BLUE' | 'BROWN'
    suit: 'CLUBS' | 'DIAMONDS' | 'HEARTS' | 'SPADES'
    action: (PLAYER[]) -> void
}

'''
    Card ID's will reduce network request payload size
    by allowinng us to send one INT to identify a card
'''
cards: Map < INT, CARD >


'''
    When a player plays a card, the target of the card will be included 
'''
