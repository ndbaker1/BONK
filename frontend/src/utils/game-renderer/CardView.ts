import { COLORS } from './colors'
import { Dimensions } from './game-renderer'
import { roundRect } from './utils'

export const CardView = {
  render(canvasContext: CanvasRenderingContext2D, dim: Dimensions): void {
    // canvas rendering colors & configs
    canvasContext.lineWidth = 5
    canvasContext.strokeStyle = COLORS.HUDBorder
    canvasContext.fillStyle = COLORS.HUDBg

    roundRect(canvasContext, dim.x, dim.y, dim.width, dim.height, 10)
  }
}