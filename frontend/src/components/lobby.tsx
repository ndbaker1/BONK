import React from 'react'
import { Button, IconButton, InputAdornment, List, ListItem, ListItemText, TextField, Tooltip } from '@material-ui/core'
import { FileCopyOutlined } from '@material-ui/icons'

import { useSessionData } from '../providers/session.provider'
import { useServerConnection } from '../providers/server-connecton.provider'
import Container from './container'
import { Screen, useScreen } from '../providers/screen.provider'

export default function LobbyComponent(): JSX.Element {

  const { connection } = useServerConnection()
  const { log, getSession, getUser, getUsers } = useSessionData()
  const { setScreen } = useScreen()

  return (
    <Container>
      <TextField label="UserID" variant="outlined" value={getUser()} />

      <div style={{ display: 'flex', flexDirection: 'column', margin: 'auto' }}>
        <Button onClick={() => {
          connection?.disconnect()
          setScreen(Screen.Login)
        }}> Disconnect </Button>

        <TextField
          id="session-id"
          label="Session ID"
          variant="outlined"
          value={getSession()}
          InputProps={{
            endAdornment:
              <InputAdornment position="end">
                <Tooltip title="Copy to Clipboard">
                  <IconButton
                    onClick={() =>
                      navigator.clipboard.writeText(getSession())
                        .then(() => log('Copied SessionID: ' + getSession()))
                    }
                  > <FileCopyOutlined />
                  </IconButton>
                </Tooltip>
              </InputAdornment>
          }}
        />
        <Button onClick={() => {
          connection?.leave_session()
        }}>  Leave Session </Button>

        <Button onClick={() => {
          connection?.startGame()
        }}>  Start Game </Button>

        <UserList users={getUsers()} />

      </div>
    </Container>
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