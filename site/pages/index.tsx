import Head from 'next/head'
import { useEffect, useRef, useState } from 'react'
import { ClientConnection } from '../utils/websocket-client'
import { Button, IconButton, InputAdornment, List, ListItem, ListItemText, Paper, Slide, Snackbar, TextField } from '@material-ui/core'
import { ServerEvent, ServerEventCodes } from 'utils/event-types'
import { environment } from 'environment'
import { FileCopyOutlined } from '@material-ui/icons'

// wake up app using the health endpoint
if (environment.healthCheck) {
  fetch(`${environment.http_or_https}://${environment.apiDomain}/health`).then(() => console.log('health check passed'))
}

export default function Home(): JSX.Element {
  // Make sure to utilize <useRef()> because the object will be stale in closures
  const clientRef = useRef<ClientConnection>(
    new ClientConnection({
      [ServerEventCodes.ClientJoined]: (response: ServerEvent) => {
        console.log(response)
        if (response.client_id == userRef.current) {
          setActiveSession(response.session_id || '')
          setNotification({
            open: true, text: response.session_client_ids?.length == 1
              ? 'Created New Session!'
              : 'Joined Session!'
          })
        } else {
          setNotification({ open: true, text: 'User ' + response.client_id + ' Joined!' })
        }
        setUsers(response.session_client_ids || [])
      },
      [ServerEventCodes.ClientLeft]: (response: ServerEvent) => {
        if (response.client_id == userRef.current) {
          setActiveSession('')
          setUsers([])
          setNotification({ open: true, text: 'Left the Session.' })
        } else {
          setNotification({ open: true, text: 'User ' + response.client_id + ' Left!' })
        }
        setUsers(curUsers => curUsers.filter(id => id != response.client_id))
      },
      [ServerEventCodes.GameStarted]: () => {
        setNotification({ open: true, text: 'Game is starting!' })
      },
      [ServerEventCodes.DataResponse]: (response: ServerEvent) => {
        setActiveSession(response.session_id || '')
        setUsers(response.session_client_ids || [])
        setNotification({ open: true, text: 'Resumed Previous Session!' })
      },
      [ServerEventCodes.InvalidSessionID]: (response: ServerEvent) => {
        setNotification({ open: true, text: response.session_id + ' is not a valid Session ID' })
      },
    })
  )

  const [activeSession, setActiveSession] = useState('')

  const [user, setUser] = useState('test_user')
  const userRef = useRef(user)
  useEffect(() => { userRef.current = user }, [user])

  const [inputSession, setInputSession] = useState('')
  const [notification, setNotification] = useState({ open: false, text: ' ' })
  const [users, setUsers] = useState<string[]>([])

  return (
    <div style={{ width: '100vw', height: '100vh', backgroundImage: 'linear-gradient(30deg, #16222A, #3A6073)' }}>
      <Head>
        <title>BONK</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <div style={{ display: 'flex', flexDirection: 'column', margin: 'auto', width: '100%', height: '100%' }}>

        <Paper elevation={7} style={{ display: 'flex', flexDirection: 'column', margin: 'auto', padding: '2rem' }}>
          <div style={{ display: 'flex', flexDirection: 'column', margin: 'auto' }}>

            <TextField label="UserID" variant="outlined" value={user} onChange={(event) => setUser(event.target.value)} />
            <Button onClick={() => {
              setNotification({ open: true, text: 'Connecting...' })
              clientRef.current.connect(user, {
                open: () => {
                  clientRef.current.getState()
                  setNotification({ open: true, text: 'Connected!' })
                },
                error: () => setNotification({ open: true, text: 'Error: ID may already be taken.' })
              })
            }}> Connect </Button>

            <div style={{ display: clientRef.current.isOpen() ? 'flex' : 'none', flexDirection: 'column', margin: 'auto' }}>
              <Button onClick={() => {
                clientRef.current.disconnect()
              }}> Disconnect </Button>

              {
                activeSession.length != 0
                  ? (
                    <>
                      <TextField id="session-id" label="Session ID" variant="outlined" value={activeSession}
                        InputProps={{
                          endAdornment: <InputAdornment position="end">
                            <IconButton
                              onClick={() =>
                                navigator.clipboard.writeText(activeSession)
                                  .then(() => setNotification({ open: true, text: 'Copied SessionID: ' + activeSession }))
                              }
                            >
                              <FileCopyOutlined />
                            </IconButton></InputAdornment>
                        }}
                      />
                      <Button onClick={() => {
                        clientRef.current.leave_session()
                      }}>  Leave Session </Button>
                    </>
                  )
                  : (
                    <>
                      < TextField label="Session ID" variant="outlined" value={inputSession} onChange={(event) => setInputSession(event.target.value)} />
                      <Button onClick={() =>
                        clientRef.current.join_session(inputSession, (error) => setNotification({ open: true, text: error }))
                      }>  Join Session </Button>

                      <Button onClick={() => {
                        clientRef.current.create_session()
                      }}> Create Session </Button>
                    </>
                  )
              }

              <UserList users={users} />

            </div>
          </div>
        </Paper>

        <Snackbar
          anchorOrigin={{ vertical: 'top', horizontal: 'center' }}
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


function UserList({ users }: { users: string[] }) {
  return (
    <List dense={true}>
      {users.map((user, i) => (
        <ListItem key={i}>
          <ListItemText primary={user} />
        </ListItem>
      ))}
    </List>
  )
}
