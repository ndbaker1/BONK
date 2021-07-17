import { Dimensions } from './game-renderer'

export const CardPile = {
  Discard: {
    render(): void {
      //
    },
    StackViewer: {
      render(canvasContext: CanvasRenderingContext2D, dim: Dimensions): void {
        canvasContext.fillRect(dim.x, dim.y, dim.width, dim.height)
      }
    }
  },
  Draw: {
    render(): void {
      //
    }
  },
}
