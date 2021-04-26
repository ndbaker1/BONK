import Head from 'next/head'
import { useEffect } from 'react'
import { w3cwebsocket as W3CWebSocket } from 'websocket'

const socket = new W3CWebSocket('ws://127.0.0.1:8000/ws/f31c43d9dbf34a538d1a4d9322e7dde9')
//curl -X POST 'http://localhost:8000/register' -H 'Content-Type: application/json' -d '{ "user_id": 1 }

export default function Home(): JSX.Element {
  useEffect(() => {
    socket.onopen = () => {
      console.log('connected to websocket!')
    }
  })

  return (
    <div>
      <Head>
        <title>Create Next App</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>
    </div>
  )
}
