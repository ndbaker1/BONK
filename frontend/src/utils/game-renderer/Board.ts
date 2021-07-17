import { COLORS } from './colors'
import { Area } from './game-renderer'

export const Board = {
  render(canvasContext: CanvasRenderingContext2D, area: Area): void {
    // create gradient for the background of the board
    const boardTexture = canvasContext.createLinearGradient(0, 0, 0, area.height)
    boardTexture.addColorStop(0, COLORS.BoardGradient1)
    boardTexture.addColorStop(1, COLORS.BoardGradient2)
    // Fill with gradient
    canvasContext.fillStyle = boardTexture
    canvasContext.fillRect(0, 0, area.width, area.height)
  }
}

