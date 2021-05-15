import React from 'react'
import { Helmet } from 'react-helmet'
import GameComponent from '../components/game'

export default function Home(): JSX.Element {
  return (
    <div style={{ width: '100vw', height: '100vh', backgroundImage: 'linear-gradient(30deg, #16222A, #3A6073)' }}>
      <Helmet>
        <meta charSet="utf-8" />
        <title>BONK</title>
        <link rel="icon" href="./favicon.ico" />
      </Helmet>
      <GameComponent />
    </div>
  )
}