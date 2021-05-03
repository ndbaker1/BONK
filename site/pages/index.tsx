import Head from 'next/head'
import { useEffect, useRef, useState } from 'react'
import { ClientConnection } from '../utils/websocket-client'
import { Button, Paper, Slide, Snackbar, TextField } from '@material-ui/core'
import { ServerEvent, ServerEventCodes } from 'utils/event-types'

export default function Home(): JSX.Element {
  // Make sure to utilize <useRef()> because the object will be stale in closures
  const clientRef = useRef<ClientConnection>(
    new ClientConnection({
      [ServerEventCodes.ClientJoined]: (response: ServerEvent) => {
        console.log(response, userRef.current)
        if (response.client_id == userRef.current) {
          setActiveSession(response.session_id || '')
        } else {
          setNotification({ open: true, text: 'User ' + response.client_id + ' Joined!' })
        }
      },
      [ServerEventCodes.ClientLeft]: (response: ServerEvent) => {
        if (response.client_id == userRef.current) {
          setActiveSession('')
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

  const [activeSession, setActiveSession] = useState('')

  const [user, setUser] = useState('test_user')
  const userRef = useRef(user)
  useEffect(() => { userRef.current = user }, [user])

  const [inputSession, setInputSession] = useState('')
  const [notification, setNotification] = useState({ open: false, text: ' ' })

  return (
    <div style={{ width: '100vw', height: '100vh', backgroundImage: 'linear-gradient(30deg, #16222A, #3A6073)' }}>
      <Head>
        <title>Create Next App</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <div style={{ display: 'flex', flexDirection: 'column', margin: 'auto', width: '100%', height: '100%' }}>

        <Paper elevation={7} style={{ display: 'flex', flexDirection: 'column', margin: 'auto', padding: '2rem' }}>
          <div style={{ display: 'flex', flexDirection: 'column', margin: 'auto' }}>

            <TextField label="UserID" variant="outlined" value={user} onChange={(event) => setUser(event.target.value)} />
            <Button onClick={() => {
              clientRef.current.connect(user, {
                open: () => setNotification({ open: true, text: 'Connected!' }),
                error: () => setNotification({ open: true, text: 'Error: ID may already be taken.' })
              })
            }}> Connect </Button>

            <div style={{ display: clientRef.current.isOpen() ? 'flex' : 'none', flexDirection: 'column', margin: 'auto' }}>
              <Button onClick={() => {
                clientRef.current.disconnect()
              }}> Disconnect </Button>

              <TextField label="Active Session" variant="outlined" value={activeSession} />
              <Button onClick={() => {
                clientRef.current.leave_session()
              }}>  Leave Sessions </Button>

              <TextField label="Target Session" variant="outlined" value={inputSession} onChange={(event) => setInputSession(event.target.value)} />
              <Button onClick={() => {
                clientRef.current.join_session(inputSession)
              }}>  Join Session </Button>

              <Button onClick={() => {
                clientRef.current.create_session()
              }}> Create Session </Button>
            </div>
          </div>
        </Paper>

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