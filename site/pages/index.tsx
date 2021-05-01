import Head from 'next/head'
import { useState } from 'react'
import { ClientConnection } from '../utils/websocket-client'

export default function Home(): JSX.Element {
  const client = new ClientConnection()
  const [user, setUser] = useState('test_user')
  const [sessionId, setSessionId] = useState('')
  return (
    <div>
      <Head>
        <title>Create Next App</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <div style={{ display: 'flex', flexDirection: 'column', margin: 'auto' }}>

        <div style={{ display: 'flex', flexDirection: 'column', margin: 'auto' }}>
          <input value={user} onChange={(event) => setUser(event.target.value)}></input>
          <button onClick={() => {
            client.connect(user, () => { console.log('after connect procedure') })
          }}> Connect </button>

          <button onClick={() => {
            client.play_card(2)
          }}> Card 2 </button>

          <button onClick={() => {
            client.leave_session()
          }}>  Leave Sessions </button>

          <input value={sessionId} onChange={(event) => setSessionId(event.target.value)}></input>
          <button onClick={() => {
            client.join_session(sessionId)
          }}>  Join Session </button>

          <button onClick={() => {
            client.create_session()
          }}> Create Session </button>
        </div>
      </div>
    </div>
  )
}
