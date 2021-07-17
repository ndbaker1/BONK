import { Card } from '../shared-types'
import { CardView } from './CardView'
import { Dimensions } from './game-renderer'

export const Field = {
  render(canvasContext: CanvasRenderingContext2D, cards: Card[], cardSpace: Dimensions, cardWidth = 140): void {
    cards = [
      { name: 1, rank: 1, suit: 1 },
      { name: 1, rank: 1, suit: 1 },
      { name: 1, rank: 1, suit: 1 },
    ]
    cards.forEach((_, i) => {
      CardView.render(canvasContext, {
        ...cardSpace,
        width: cardWidth,
        x: cardSpace.x - cardWidth / 2 + (i + 1) * cardSpace.width / (cards.length + 1)
      })
    })
  }
}


