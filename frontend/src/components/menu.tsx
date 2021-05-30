import React from 'react'
import { Button, IconButton, InputAdornment, TextField, Tooltip } from '@material-ui/core'
import ForwardIcon from '@material-ui/icons/Forward'
import PlayForWorkIcon from '@material-ui/icons/PlayForWork'

import { useSessionData } from '../providers/session.provider'
import { useServerConnection } from '../providers/server-connecton.provider'

import Container from './container'
import { useNotification } from '../providers/notification.provider'


export default function MenuComponent(): JSX.Element {
  const [joining, setJoining] = React.useState(false)
  return (
    <Container>
      {
        joining
          ? <JoinSessionComponent goBack={() => setJoining(false)} />
          : <NavigateComponent join={() => setJoining(true)} />
      }
    </Container>
  )
}

function JoinSessionComponent({ goBack }: { goBack: () => void }) {
  const { connection } = useServerConnection()
  const { setNotification } = useNotification()

  const [inputSession, setInputSession] = React.useState('')

  return (
    <>
      <div>
        <TextField
          label="Session ID"
          variant="outlined"
          value={inputSession}
          onChange={event => setInputSession(event.target.value)}
          InputProps={{
            endAdornment:
              <InputAdornment position="end">
                <Tooltip title="Join">
                  <IconButton
                    onClick={() => connection?.join_session(inputSession, (error) => setNotification(error))}
                  > <ForwardIcon />
                  </IconButton>
                </Tooltip>
                <Tooltip title="Pull From Clipboard">
                  <IconButton
                    onClick={() => navigator.clipboard.readText()
                      .then(session => {
                        setInputSession(session)
                        connection?.join_session(session, (error) => setNotification(error))
                      })
                    }
                  > <PlayForWorkIcon />
                  </IconButton>
                </Tooltip>
              </InputAdornment>
          }} />
      </div>
      <Button onClick={() => goBack()}> Back </Button>
    </>
  )
}

function NavigateComponent({ join }: { join: () => void }) {
  const { connection } = useServerConnection()
  const { getUser } = useSessionData()

  return (
    <>
      <TextField label="UserID" variant="outlined" value={getUser()} />
      <Button onClick={() => connection?.disconnect()}> Disconnect </Button>
      <Button onClick={() => join()}> Join Session </Button>
      <Button onClick={() => connection?.create_session()}> Create Session </Button>
    </>
  )
}