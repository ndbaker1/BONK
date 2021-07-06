import { PlayerData } from './shared-types'

const COLORS = {
  HUDBg: '#FAECDE',
  HUDBorder: '#7B3018',
  BlueCardsBorder: '#6A8DAD',
  GreenCardBorder: '#74956C',
  BrownCardBorder: '#CF9B67',
  BoardGradient1: '#F0DCB4',
  BoardGradient2: '#D8AE88',
}
// background: linear-gradient(359.88deg, #D8AE88 0.1%, #E9CEA6 28.42%, #EFD2A5 51.08%, #F0DCB4 87.13%);

export class GameRenderer {

  dimensions = { height: 0, width: 0 }
  canvasContext!: CanvasRenderingContext2D

  render(): void {
    if (!this.canvasContext) console.error('The Rendering Canvas has not beed initialized!')

    this.board()
    this.players({} as PlayerData)
    this.HUD()
    this.discardPile()
    this.drawPile()
  }

  private players(playerData: PlayerData) {
    // 
  }

  // details which always appear on the bottom left of the screen...
  private HUD() {
    // HUD takes up 15% of the screen height
    const h = this.dimensions.height * 0.15
    // HUD takes up 30% of the screen width
    const w = this.dimensions.width * 0.3
    // top-left of the HUD Area
    const { x, y } = { x: 0, y: this.dimensions.height - h }
    // bottom-right of the HUD Area
    const { r, b } = { r: x + w, b: y + h }
    // radius of the top-right HUD corner
    const cornerRadius = 12
    // canvas rendering colors & configs
    this.canvasContext.lineWidth = 8
    this.canvasContext.strokeStyle = COLORS.HUDBorder
    this.canvasContext.fillStyle = COLORS.HUDBg

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

  private board() {
    // create gradient for the background of the board
    const boardTexture = this.canvasContext.createLinearGradient(0, 0, 0, this.dimensions.height)
    boardTexture.addColorStop(0, COLORS.BoardGradient1)
    boardTexture.addColorStop(1, COLORS.BoardGradient2)
    // Fill with gradient
    this.canvasContext.fillStyle = boardTexture
    this.canvasContext.fillRect(0, 0, this.dimensions.width, this.dimensions.height)
  }
}