import React from 'react'
import { ServerConnection } from '../utils/websocket-client'
import { Slide, Snackbar } from '@material-ui/core'
import { ServerEventCode, ServerEvent } from '../utils/shared-types'
import { environment } from '../environment'

import { useSessionData } from '../providers/session.provider'
import { Screen, useScreen } from '../providers/screen.provider'
import { useServerConnection } from '../providers/server-connecton.provider'
import { useNotification } from '../providers/notification.provider'

import LobbyComponent from './lobby'
import LoginComponent from './login'
import MenuComponent from './menu'
import GameComponent from './game'


export default function AppComponent(): JSX.Element {

  const { getScreen, setScreen } = useScreen()
  const { setConnection } = useServerConnection()
  const { setSession, getUser, getUsers, setUsers } = useSessionData()
  const { notification, setNotification } = useNotification()

  // run once on init
  React.useEffect(() => {
    if (environment.healthCheck) // wake up app using the health endpoint
      fetch(`${environment.http_or_https}://${environment.apiDomain}/health`)
        .then(() => console.log('health check passed'))

    setConnection(
      new ServerConnection({
        [ServerEventCode.ClientJoined]: (response: ServerEvent) => {
          if (response.data?.client_id == getUser()) {
            setSession(response.data?.session_id || '')
            setNotification(response.data?.session_client_ids?.length == 1
              ? 'Created New Session!'
              : 'Joined Session!'
            )
            setScreen(Screen.Lobby)
          } else {
            setNotification('User ' + response.data?.client_id + ' Joined!')
          }
          setUsers(response.data?.session_client_ids || [])
        },
        [ServerEventCode.ClientLeft]: (response: ServerEvent) => {
          if (response.data?.client_id == getUser()) {
            setSession('')
            setUsers([])
            setNotification('Left the Session.')
            setScreen(Screen.Menu)
          } else {
            setNotification('User ' + response.data?.client_id + ' Left!')
          }
          setUsers(getUsers().filter(id => id != response.data?.client_id))
        },
        [ServerEventCode.GameStarted]: (response: ServerEvent) => {
          // setGameData(response.data?.game_data)
          // setPlayerData(response.data?.player_data)
          setNotification('Game is starting!')
          setScreen(Screen.Game)
        },
        [ServerEventCode.DataResponse]: (response: ServerEvent) => {
          setSession(response.data?.session_id || '')
          setUsers(response.data?.session_client_ids || [])
          setNotification('Resumed Previous Session!')
          // setGameData(response.data?.game_data)
          // setPlayerData(response.data?.player_data)
          setScreen(Screen.Lobby) // set to different state depending on gamedata
        },
        [ServerEventCode.TurnStart]: (response: ServerEvent) => {
          setNotification(response.data?.session_id + ' is not a valid Session ID')
        },
        [ServerEventCode.LogicError]: (response: ServerEvent) => {
          setNotification(response.message || '')
        },
      })
    )
  }, [])

  return (
    <div style={{ display: 'flex', flexDirection: 'column', margin: 'auto', width: '100%', height: '100%' }}>
      <ScreenRouter screen={getScreen()} />
      <Snackbar
        anchorOrigin={{ vertical: 'top', horizontal: 'center' }}
        open={notification !== ''}
        onClose={() => setNotification('')}
        TransitionComponent={Slide}
        message={notification}
        autoHideDuration={2000} />
    </div >
  )
}

function ScreenRouter({ screen }: { screen: Screen }) {
  switch (screen) {
    case Screen.Login: return <LoginComponent />
    case Screen.Menu: return <MenuComponent />
    case Screen.Lobby: return <LobbyComponent />
    case Screen.Game: return <GameComponent />
  }
}
