import React, { useEffect, useRef } from 'react'
import { useGameData } from '../providers/game.provider'
import { GameRenderer } from '../utils/game-renderer'

const gameRenderer = new GameRenderer()

/**
 * Renders the Game according to the State
 */
export default function GameComponent(): JSX.Element {
  const { data } = useGameData()
  const canvasref = useRef<HTMLCanvasElement>(null)

  useEffect(() => {
    const canvas = canvasref.current
    const ctx = canvas?.getContext('2d')
    if (canvas && ctx) {
      gameRenderer.canvasContext = ctx
      gameRenderer.dimensions = {
        height: canvas.clientHeight,
        width: canvas.clientWidth,
      }
      gameRenderer.render()
    }
  }, [])

  return (
    <canvas
      ref={canvasref} width={1000} height={750}
      style={{
        backgroundColor: '#fbf7f5',
        position: 'absolute',
        display: 'block',
        margin: 'auto',
        top: 0,
        bottom: 0,
        left: 0,
        right: 0,
      }} />
  )
}