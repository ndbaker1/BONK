import { Card, PlayerData } from './shared-types'

const COLORS = {
  HUDBg: '#FAECDE',
  HUDBorder: '#7B3018',
  BlueCardsBorder: '#6A8DAD',
  GreenCardBorder: '#74956C',
  BrownCardBorder: '#CF9B67',
  BoardGradient1: '#F0DCB4',
  BoardGradient2: '#D8AE88',
}

export class GameRenderer {

  dimensions = { height: 0, width: 0 }
  canvasContext!: CanvasRenderingContext2D

  render(): void {
    if (!this.canvasContext) console.error('The Rendering Canvas has not beed initialized!')

    Board.render(this.canvasContext, this.dimensions.width, this.dimensions.height)
    this.players({} as PlayerData)
    this.HUD()
    Hand.render(this.canvasContext, [], { x: this.dimensions.width - 550, y: this.dimensions.height - 250, w: 500, h: 200 })
    Field.render(this.canvasContext, [], { x: this.dimensions.width / 2 - 400, y: this.dimensions.height - 250 - 200, w: 800, h: 200 })
    CardPile.Discard.render()
    CardPile.Draw.render()
  }

  private players(playerData: PlayerData) {
    // 
  }

  // details which always appear on the bottom left of the screen...
  private HUD() {
    // canvas rendering colors & configs
    this.canvasContext.lineWidth = 4
    this.canvasContext.strokeStyle = COLORS.HUDBorder
    this.canvasContext.fillStyle = COLORS.HUDBg

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
    this.canvasContext.fill()
    this.canvasContext.stroke()
  }
}

const CardPile = {
  Discard: {
    render() {
      //
    },
    StackViewer: {
      render(canvasContext: CanvasRenderingContext2D, x: number, y: number, w: number, h: number) {
        canvasContext.fillRect(x, y, w, h)
      }
    }
  },
  Draw: {
    render() {
      //
    }
  },
}

const Board = {
  render(canvasContext: CanvasRenderingContext2D, width: number, height: number) {
    // create gradient for the background of the board
    const boardTexture = canvasContext.createLinearGradient(0, 0, 0, height)
    boardTexture.addColorStop(0, COLORS.BoardGradient1)
    boardTexture.addColorStop(1, COLORS.BoardGradient2)
    // Fill with gradient
    canvasContext.fillStyle = boardTexture
    canvasContext.fillRect(0, 0, width, height)
  }
}

const Hand = {
  render(canvasContext: CanvasRenderingContext2D, cards: Card[], cardArea: { x: number, y: number, w: number, h: number }, cardWidth = 140) {
    cards = [
      { name: 1, rank: 1, suit: 1 },
      { name: 1, rank: 1, suit: 1 },
      { name: 1, rank: 1, suit: 1 },
      { name: 1, rank: 1, suit: 1 },
      { name: 1, rank: 1, suit: 1 },
    ]
    cards.forEach((_, i) => {
      CardView.render(canvasContext, { ...cardArea, w: cardWidth, x: cardArea.x + cardArea.w - (i + 3) * cardWidth / 3 })
    })
  }
}

const Field = {
  render(canvasContext: CanvasRenderingContext2D, cards: Card[], cardArea: { x: number, y: number, w: number, h: number }, cardWidth = 140) {
    cards = [
      { name: 1, rank: 1, suit: 1 },
      { name: 1, rank: 1, suit: 1 },
      { name: 1, rank: 1, suit: 1 },
    ]
    cards.forEach((_, i) => {
      CardView.render(canvasContext, {
        ...cardArea,
        w: cardWidth,
        x: cardArea.x - cardWidth / 2 + (i + 1) * cardArea.w / (cards.length + 1)
      })
    })
  }
}

const CardView = {
  render(canvasContext: CanvasRenderingContext2D, dim: { x: number, y: number, w: number, h: number }) {
    // canvas rendering colors & configs
    canvasContext.lineWidth = 5
    canvasContext.strokeStyle = COLORS.HUDBorder
    canvasContext.fillStyle = COLORS.HUDBg

    roundRect(canvasContext, dim.x, dim.y, dim.w, dim.h, 10)
  }
}

function roundRect(canvasContext: CanvasRenderingContext2D, x: number, y: number, w: number, h: number, radius: number) {
  const r = x + w
  const b = y + h
  canvasContext.beginPath()
  canvasContext.moveTo(x + radius, y)
  canvasContext.lineTo(r - radius, y)
  canvasContext.quadraticCurveTo(r, y, r, y + radius)
  canvasContext.lineTo(r, y + h - radius)
  canvasContext.quadraticCurveTo(r, b, r - radius, b)
  canvasContext.lineTo(x + radius, b)
  canvasContext.quadraticCurveTo(x, b, x, b - radius)
  canvasContext.lineTo(x, y + radius)
  canvasContext.quadraticCurveTo(x, y, x + radius, y)
  canvasContext.fill()
  canvasContext.stroke()
}