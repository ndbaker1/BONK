export function roundRect(
  canvasContext: CanvasRenderingContext2D,
  x: number,
  y: number,
  w: number,
  h: number,
  radius: number
): void {
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

