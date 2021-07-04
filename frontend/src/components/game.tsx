import { Autorenew, Block } from '@material-ui/icons'
import React, { useEffect, useRef, useState } from 'react'
import { useGameData } from '../providers/game.provider'

/**function size() {
  const {width, setWidth} = useState(0);
}
**/
/**
 * Renders the Game according to the State
 * 
 */
export default function GameComponent(): JSX.Element {
  const { data } = useGameData()
  const canvasref = useRef<HTMLCanvasElement>(null)
  useEffect(()=>{
    const canvas = canvasref.current
    let ctx = canvas?.getContext("2d")
    const roundRect = ({ x, y, w, h } : {x : number, y : number, w : number, h : number}, radius = { tr: 4, br: 4, bl: 4, tl: 4 }, color = '#000000') => {
      const r = x + w;
      const b = y + h;
      if (ctx) {
        ctx.beginPath();
        ctx.fillStyle = color;
        ctx.moveTo(x + radius.tl, y);
        ctx.lineTo(r - radius.tr, y);
        ctx.quadraticCurveTo(r, y, r, y + radius.tr);
        ctx.lineTo(r, y + h - radius.br);
        ctx.quadraticCurveTo(r, b, r - radius.br, b);
        ctx.lineTo(x + radius.bl, b);
        ctx.quadraticCurveTo(x, b, x, b - radius.bl);
        ctx.lineTo(x, y + radius.tl);
        ctx.quadraticCurveTo(x, y, x + radius.tl, y);
        ctx.closePath();
        ctx.stroke();
        ctx.fill();
        console.log("Rect run funct")
      }

     
    }
    const rectInfo = { x: 0, y: 650, w: 150, h: 100}
    const rectRad = {}
    roundRect(rectInfo)
  }
  , [])


  return (
    <canvas
    ref = {canvasref} width={1000} height={750}
    style={{
      backgroundColor: '#fbf7f5',
      position: 'absolute',
      display: 'block',
      margin: 'auto',
      top: 0,
      bottom: 0,
      left: 0,
      right: 0,
      }}/>
  )
}


