import React from 'react'
import { ClientConnection } from '../utils/websocket-client'
import { Button, IconButton, InputAdornment, List, ListItem, ListItemText, Paper, Slide, Snackbar, TextField } from '@material-ui/core'
import { ServerEventCode, GameData, CardName, ServerEvent, PlayerData, CardSuit, CardRank } from '../utils/shared-types'
import { FileCopyOutlined, PinDropSharp } from '@material-ui/icons'
import { environment } from '../environment'


export default function GameComponent(): JSX.Element {
  // run once on init
  React.useEffect(() => {
    if (environment.healthCheck) // wake up app using the health endpoint
      fetch(`${environment.http_or_https}://${environment.apiDomain}/health`).then(() => console.log('health check passed'))
  }, [])

  // Make sure to utilize <useRef()> because the object will be stale in closures
  const clientRef = React.useRef<ClientConnection>(
    new ClientConnection({
      [ServerEventCode.ClientJoined]: (response: ServerEvent) => {
        if (response.data?.client_id == userRef.current) {
          setActiveSession(response.data.session_id || '')
          setNotification({
            open: true, text: response.data.session_client_ids?.length == 1
              ? 'Created New Session!'
              : 'Joined Session!'
          })
        } else {
          setNotification({ open: true, text: 'User ' + response.data?.client_id + ' Joined!' })
        }
        setUsers(response.data?.session_client_ids || [])
      },
      [ServerEventCode.ClientLeft]: (response: ServerEvent) => {
        if (response.data?.client_id == userRef.current) {
          setActiveSession('')
          setUsers([])
          setNotification({ open: true, text: 'Left the Session.' })
        } else {
          setNotification({ open: true, text: 'User ' + response.data?.client_id + ' Left!' })
        }
        setUsers(curUsers => curUsers.filter(id => id != response.data?.client_id))
      },
      [ServerEventCode.GameStarted]: (response: ServerEvent) => {
        setGameData(response.data?.game_data)
        setPlayerData(response.data?.player_data)
        setNotification({ open: true, text: 'Game is starting!' })
      },
      [ServerEventCode.DataResponse]: (response: ServerEvent) => {
        setActiveSession(response.data?.session_id || '')
        setUsers(response.data?.session_client_ids || [])
        setNotification({ open: true, text: 'Resumed Previous Session!' })
        setGameData(response.data?.game_data)
        setPlayerData(response.data?.player_data)
      },
      [ServerEventCode.TurnStart]: (response: ServerEvent) => {
        setNotification({ open: true, text: response.data?.session_id + ' is not a valid Session ID' })
      },
      [ServerEventCode.LogicError]: (response: ServerEvent) => {
        setNotification({ open: true, text: response.message || '' })
      },
    })
  )


  const [user, setUser] = React.useState('test_user')
  const userRef = React.useRef(user)
  React.useEffect(() => { userRef.current = user }, [user])

  const [inputSession, setInputSession] = React.useState('')
  const [activeSession, setActiveSession] = React.useState('')

  const [notification, setNotification] = React.useState({ open: false, text: ' ' })
  const [users, setUsers] = React.useState<string[]>([])

  const [gameData, setGameData] = React.useState<GameData>()
  const [playerData, setPlayerData] = React.useState<PlayerData>()


  return (
    <div style={{ display: 'flex', flexDirection: 'column', margin: 'auto', width: '100%', height: '100%' }}>
      {
        !gameData
          ? (
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

                          <Button onClick={() => {
                            clientRef.current.startGame()
                          }}>  Start Game </Button>
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
          )
          : (
            <div style={{ display: 'flex', margin: 'auto 0' }}>
              <Paper elevation={7} style={{ display: 'flex', flexDirection: 'column', margin: 'auto', padding: '2rem' }}>
                <div style={{ display: 'flex', flexDirection: 'column', margin: 'auto' }}>
                  <TextField id="session-id" label="Session ID" variant="outlined" value={activeSession} />
                  <Button onClick={() => {
                    clientRef.current.leave_session()
                    setGameData(undefined)
                  }}>  Leave Session </Button>

                  {/* <Button onClick={() => {
                    clientRef.current.play_card([{ name: CardName.Bang, suit: CardSuit.Clubs, rank: CardRank.A }], [])
                  }}> Bang </Button> */}

                  <UserList users={users} />
                </div>
              </Paper>
              <Paper elevation={7} style={{ display: 'flex', flexDirection: 'column', margin: 'auto', padding: '2rem' }}>
                <JSONDataViewer data={[playerData, gameData]} />
              </Paper>
            </div>
          )
      }
      <Snackbar
        anchorOrigin={{ vertical: 'top', horizontal: 'center' }}
        open={notification.open}
        onClose={() => setNotification({ ...notification, open: false })}
        TransitionComponent={Slide}
        message={notification.text}
        autoHideDuration={2000}
      />
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

function JSONDataViewer({ data }: { data: any[] }) {
  return (
    <div style={{ display: 'flex', flexDirection: 'row' }}>
      {
        data.map((d, i) => (
          <div key={i}>
            <pre>{JSON.stringify(d, null, 2)}</pre>
          </div>
        ))
      }
    </div>
  )
}
