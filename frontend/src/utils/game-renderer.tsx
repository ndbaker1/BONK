import { PlayerData } from './shared-types'

export class GameRenderer {
  dimensions = { height: 0, width: 0 }
  canvasContext!: CanvasRenderingContext2D

  render(): void {
    if (!this.canvasContext) console.error('The Rendering Canvas has not beed initialized!')

    this.drawPlayers({} as PlayerData)
    this.HUD()
    this.discardPile()
    this.drawPile()
  }

  private drawPlayers(playerData: PlayerData) {
    // 
  }

  // details which always appear on the bottom left of the screen...
  private HUD() {
    // HUD takes up 15% of the screen height
    const h = this.dimensions.height * 0.15
    // HUD takes up 30% of the screen width
    const w = this.dimensions.height * 0.3
    // top-left of the HUD Area
    const { x, y } = { x: 0, y: this.dimensions.height - h }
    // bottom-right of the HUD Area
    const { r, b } = { r: x + w, b: y + h }
    // radius of the top-right HUD corner
    const cornerRadius = 12
    // canvas rendering colors & configs
    this.canvasContext.lineWidth = 8
    this.canvasContext.strokeStyle = '#7B3018'
    this.canvasContext.fillStyle = '#FAECDE'

    /*******************
     * Start Draw
     *******************/
    // starts at top-left
    this.canvasContext.beginPath()
    this.canvasContext.moveTo(x, y)
    // curve top-right
    this.canvasContext.lineTo(r - cornerRadius, y)
    this.canvasContext.quadraticCurveTo(r, y, r, y + cornerRadius)
    // line to bottom-right
    this.canvasContext.lineTo(r, b)
    // lines to bottom-left
    this.canvasContext.lineTo(x, b)
    // closes path to complete bottom-left back to top-left
    this.canvasContext.closePath()
    // color shape/path
    this.canvasContext.stroke()
    this.canvasContext.fill()
  }

  private discardPile() {
    //
  }

  private drawPile() {
    //
  }

}