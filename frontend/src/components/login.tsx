import React from 'react'
import { useServerConnection } from '../providers/server-connecton.provider'
import { Button, Paper, TextField } from '@material-ui/core'

import { useSessionData } from '../providers/session.provider'
import { Screen, useScreen } from '../providers/screen.provider'
import { useNotification } from '../providers/notification.provider'

export default function LoginComponent(): JSX.Element {

  const { setScreen } = useScreen()
  const { connection } = useServerConnection()
  const { setUser, getUser } = useSessionData()
  const { setNotification } = useNotification()

  return (
    <Paper elevation={7} style={{ display: 'flex', flexDirection: 'column', margin: 'auto', padding: '2rem' }}>
      <TextField label="UserID" variant="outlined" value={getUser()} onChange={(event) => setUser(event.target.value)} />
      <Button onClick={() => {
        setNotification('Connecting...')
        connection?.connect(getUser(), {
          open: () => {
            setScreen(Screen.Menu)
            setNotification('Connected.')
            connection.fetchState()
          },
          error: () => {
            setNotification('Error: ID may already be taken.')
          },
          close: () => {
            setNotification('Disconnected.')
            setScreen(Screen.Login)
          },
        })
      }}> Connect </Button>
    </Paper>
  )
}