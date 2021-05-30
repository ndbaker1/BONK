import React from 'react'
import { useServerConnection } from '../providers/server-connecton.provider'
import { Button, Paper, TextField } from '@material-ui/core'

import { useSessionData } from '../providers/session.provider'
import { Screen, useScreen } from '../providers/screen.provider'

export default function LoginComponent(): JSX.Element {

  const { setScreen } = useScreen()
  const { connection } = useServerConnection()
  const { setUser, log, getUser } = useSessionData()

  return (
    <Paper elevation={7} style={{ display: 'flex', flexDirection: 'column', margin: 'auto', padding: '2rem' }}>
      <TextField label="UserID" variant="outlined" value={getUser()} onChange={(event) => setUser(event.target.value)} />
      <Button onClick={() => {
        log('Connecting...')
        connection?.connect(getUser(), {
          open: () => {
            setScreen(Screen.Menu)
            log('Connected..')
            connection.fetchSession()
          },
          error: () => {
            log('Error: ID may already be taken.')
          },
          close: () => {
            log('Disconnected..')
            setScreen(Screen.Login)
          },
        })
      }}> Connect </Button>
    </Paper>
  )
}