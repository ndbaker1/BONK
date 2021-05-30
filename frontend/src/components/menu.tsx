import React from 'react'
import { Button, TextField } from '@material-ui/core'

import { useSessionData } from '../providers/session.provider'
import { useServerConnection } from '../providers/server-connecton.provider'

import Container from './container'


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
  const { log } = useSessionData()

  const [inputSession, setInputSession] = React.useState('')

  return (
    <>
      <TextField label="Session ID" variant="outlined" value={inputSession} onChange={event => setInputSession(event.target.value)} />
      <Button onClick={() => connection?.join_session(inputSession, (error) => log(error))}> Join </Button>
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