import { COLORS } from './colors'
import { Area } from './game-renderer'

export const HUD = {
  // details which always appear on the bottom left of the screen...
  render(canvasContext: CanvasRenderingContext2D, dimensions: Area): void {
    // canvas rendering colors & configs
    canvasContext.lineWidth = 4
    canvasContext.strokeStyle = COLORS.HUDBorder
    canvasContext.fillStyle = COLORS.HUDBg

    // HUD takes up 15% of the screen height
    const h = dimensions.height * 0.15
    // HUD takes up 30% of the screen width
    const w = dimensions.width * 0.3
    // top-left of the HUD Area
    const { x, y } = { x: 0, y: dimensions.height - h }
    // bottom-right of the HUD Area
    const { r, b } = { r: x + w, b: y + h }
    // radius of the top-right HUD corner
    const cornerRadius = 12


    /*******************
     * Start Draw
     *******************/
    // starts at top-left
    canvasContext.beginPath()
    canvasContext.moveTo(x, y)
    // curve top-right
    canvasContext.lineTo(r - cornerRadius, y)
    canvasContext.quadraticCurveTo(r, y, r, y + cornerRadius)
    // line to bottom-right
    canvasContext.lineTo(r, b)
    // lines to bottom-left
    canvasContext.lineTo(x, b)
    // closes path to complete bottom-left back to top-left
    canvasContext.closePath()
    // color shape/path
    canvasContext.fill()
    canvasContext.stroke()
  }
}
