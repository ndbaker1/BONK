import Head from 'next/head'
import { useEffect, useRef, useState } from 'react'
import { ClientConnection } from '../utils/websocket-client'
import { Slide, Snackbar } from '@material-ui/core'
import { ServerEvent, ServerEventCodes } from 'utils/event-types'

export default function Home(): JSX.Element {
  // Make sure to utilize <useRef()> because the object will be stale in closures
  const clientRef = useRef<ClientConnection>(
    new ClientConnection({
      [ServerEventCodes.ClientJoined]: (response: ServerEvent) => {
        console.log(response, userRef.current)
        if (response.client_id == userRef.current) {
          setSession(response.session_id || '')
        } else {
          setNotification({ open: true, text: 'User ' + response.client_id + ' Joined!' })
        }
      },
      [ServerEventCodes.ClientLeft]: (response: ServerEvent) => {
        if (response.client_id == userRef.current) {
          setSession('')
        } else {
          setNotification({ open: true, text: 'User ' + response.client_id + ' Left!' })
        }
      },
      [ServerEventCodes.GameStarted]: () => {
        setNotification({ open: true, text: 'Game is starting!' })
      },
      [ServerEventCodes.TurnStart]: (response: ServerEvent) => {
        setNotification({ open: true, text: user == response.client_id ? 'Your Turn!' : 'User ' + response.client_id + '\'s Turn!' })
      }
    })
  )

  const [session, setSession] = useState('')
  const [user, setUser] = useState('test_user')
  const userRef = useRef(user)
  useEffect(() => {
    userRef.current = user
  }, [user])
  const [sessionId, setSessionId] = useState('')
  const [notification, setNotification] = useState({ open: false, text: ' ' })

  return (
    <div>
      <Head>
        <title>Create Next App</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <div style={{ display: 'flex', flexDirection: 'column', margin: 'auto' }}>

        <div style={{ display: 'flex', flexDirection: 'column', margin: 'auto' }}>
          <input value={user} onChange={(event) => setUser(event.target.value.toString())}></input>
          <button onClick={() => {
            clientRef.current.connect(user, {
              open: () => setNotification({ open: true, text: 'Connected!' }),
              error: () => setNotification({ open: true, text: 'Error: ID may already be taken.' })
            })
          }}> Connect </button>

          <div style={{ display: clientRef.current.isOpen() ? 'flex' : 'none', flexDirection: 'column', margin: 'auto' }}>
            <button onClick={() => {
              clientRef.current.play_card(2)
            }}> Card 2 </button>

            <p>{session}</p>
            <button onClick={() => {
              clientRef.current.leave_session()
            }}>  Leave Sessions </button>

            <input value={sessionId} onChange={(event) => setSessionId(event.target.value)}></input>
            <button onClick={() => {
              clientRef.current.join_session(sessionId)
            }}>  Join Session </button>

            <button onClick={() => {
              clientRef.current.create_session()
            }}> Create Session </button>
          </div>
        </div>

        <Snackbar
          open={notification.open}
          onClose={() => setNotification({ ...notification, open: false })}
          TransitionComponent={Slide}
          message={notification.text}
          autoHideDuration={2000}
        />
      </div>
    </div >
  )
}