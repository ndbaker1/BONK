import { Board } from './Board'
import { CardPile } from './CardPile'
import { Field } from './Field'
import { Hand } from './Hand'
import { HUD } from './HUD'
import { Players } from './Players'

export type Area = { height: number, width: number }
export type Points = { x: number, y: number }
export type Dimensions = Area & Points

export class GameRenderer {

  area: Area = { height: 0, width: 0 }
  canvasContext!: CanvasRenderingContext2D

  render(): void {
    if (!this.canvasContext) console.error('The Rendering Canvas has not beed initialized!')

    Board.render(this.canvasContext, this.area)
    Players.render(this.canvasContext, this.area, [])
    HUD.render(this.canvasContext, this.area)
    Hand.render(this.canvasContext, [], {
      x: this.area.width - 550,
      y: this.area.height - 250,
      width: 500,
      height: 200
    })
    Field.render(this.canvasContext, [], {
      x: this.area.width / 2 - 400,
      y: this.area.height - 250 - 200,
      width: 800,
      height: 200
    })
    CardPile.Discard.render()
    CardPile.Draw.render()
  }
}

