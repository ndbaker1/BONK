import React, { useEffect, useRef } from 'react'
import { useGameData } from '../providers/game.provider'
import { GameRenderer } from '../utils/game-renderer/game-renderer'

//// Renderer 
// pass a canvas context and dimensions in for rendering
const gameRenderer = new GameRenderer()

/**
 * Renders the Game according to the State
 */
export default function GameComponent(): JSX.Element {
  const { data } = useGameData()

  const canvasRef = useRef<HTMLCanvasElement>(null)
  const canvasContainerRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const canvas = canvasRef.current
    const ctx = canvas?.getContext('2d')
    // Attach the canvas to the Renderer
    if (canvas && ctx) gameRenderer.canvasContext = ctx

    // Eventlistener for Resizing the Canvas along with any page resizes
    function updateBounds() {
      if (canvasRef.current && canvasContainerRef.current) {
        const padding = 50
        // update the canvas dimensions
        canvasRef.current.width = canvasContainerRef.current.clientWidth - 2 * padding
        canvasRef.current.height = canvasContainerRef.current.clientHeight - 2 * padding
        // update the renderer dimensions
        gameRenderer.area = {
          height: canvasRef.current.clientHeight,
          width: canvasRef.current.clientWidth,
        }
      }
      gameRenderer.render() // TEMP - should be in render loop
    }

    // add the callback to the resize event listener and trigger initial update
    window.addEventListener('resize', updateBounds)
    updateBounds()
    // return a cleanup callback for the component
    return () => window.removeEventListener('resize', updateBounds)
  }, [])

  return (
    <div ref={canvasContainerRef} style={flexContainerStyles}>
      <canvas ref={canvasRef} />
    </div>
  )
}

const flexContainerStyles = {
  width: '100%',
  height: '100%',
  display: 'flex',
  placeItems: 'center',
  justifyContent: 'center'
}