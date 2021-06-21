import React, { useState } from 'react'
import { CardName, ServerEventCode, ServerEventData } from '../utils/shared-types'
import { ServerConnection } from '../utils/websocket-client'
import Container from './container'

export default function AppTestComponent(): JSX.Element {


  const [userDatas, setUserDatas] = useState<Record<'user1' | 'user2' | 'user3' | 'user4', ServerEventData | undefined>>({
    user1: {},
    user2: {},
    user3: {},
    user4: {},
  })
  function setUserData(user: 'user1' | 'user2' | 'user3' | 'user4', data: ServerEventData | undefined) {
    setUserDatas(datas => {
      datas[user] = data
      return datas
    })
  }

  const [logs, setLogs] = useState<string[]>([])
  function addLog(msg: string) { setLogs(logs => [...logs, msg]) }

  React.useEffect(() => {
    const user1 = new ServerConnection({
      [ServerEventCode.ClientJoined]: (response) => {
        if (response.data) {
          if (response.data.client_id === 'user1' && response.data.session_id) {
            addLog('successfully created session with user1 as owner')
            addLog('requesting user2, user3, and user4 to join...')
            user2.joinSession(response.data.session_id)
            user3.joinSession(response.data.session_id)
            user4.joinSession(response.data.session_id)
          } else if (response.data.session_client_ids && response.data.session_client_ids.length === 4) {
            addLog('all users joined session!')
            addLog('starting game...')
            user1.startGame()
          }
        }
      },
      [ServerEventCode.GameStarted]: (response) => {
        addLog('game started.')

        setUserData('user1', response.data)
        if (isPlayerTurn(response.data, 'user1')) {
          addLog('user1 turn.')
          if (userDatas.user1?.player_data?.hand[0]) {
            addLog(`user1 playing card ${CardName[userDatas.user1.player_data.hand[0].name]} on user2`)
            user1.playCard([userDatas.user1.player_data.hand[0]], ['user2'])
          }
        }
      },
      [ServerEventCode.LogicError]: (response) => {
        if (response.message) {
          addLog('user1 error: ' + response.message)
          if (response.message.includes('Targetted')) {
            addLog('looking for a missed...')
            const card = userDatas.user1?.player_data?.hand.find(card => card.name === CardName.Missed)
            if (card) {
              addLog('playing missed!')
              user1.playCard([card], [])
            } else {
              addLog('no missed.. taking damage.')
              user1.playCard([], [])
            }
          }
        }
      }
    })

    const user2 = new ServerConnection({
      [ServerEventCode.GameStarted]: (response) => {
        setUserData('user2', response.data)
        if (isPlayerTurn(response.data, 'user2')) {
          addLog('user2 turn.')
          if (userDatas.user2?.player_data?.hand[0]) {
            addLog(`user2 playing card ${CardName[userDatas.user2.player_data.hand[0].name]} on user3`)
            user2.playCard([userDatas.user2.player_data.hand[0]], ['user3'])
          }
        }
      },
      [ServerEventCode.LogicError]: (response) => {
        if (response.message) {
          addLog('user2 error: ' + response.message)

          if (response.message.includes('Targetted')) {
            addLog('looking for a missed...')

            const card = userDatas.user2?.player_data?.hand.find(card => card.name === CardName.Missed)
            if (card) {
              addLog('playing missed!')
              user2.playCard([card], [])
            } else {
              addLog('no missed.. taking damage.')
              user2.playCard([], [])
            }
          }
        }

      }
    })

    const user3 = new ServerConnection({
      [ServerEventCode.GameStarted]: (response) => {
        setUserData('user3', response.data)
        if (isPlayerTurn(response.data, 'user3')) {
          addLog('user3 turn.')
          if (userDatas.user3?.player_data?.hand[0]) {
            addLog(`user3 playing card ${CardName[userDatas.user3.player_data.hand[0].name]} on user4`)
            user3.playCard([userDatas.user3.player_data.hand[0]], ['user4'])
          }
        }
      },
      [ServerEventCode.LogicError]: (response) => {
        if (response.message) {
          addLog('user3 error: ' + response.message)
          if (response.message.includes('Targetted')) {
            addLog('looking for a missed...')

            const card = userDatas.user3?.player_data?.hand.find(card => card.name === CardName.Missed)
            if (card) {
              addLog('playing missed!')
              user3.playCard([card], [])
            } else {
              addLog('no missed.. taking damage.')
              user3.playCard([], [])
            }
          }
        }
      }
    })

    const user4 = new ServerConnection({
      [ServerEventCode.GameStarted]: (response) => {
        setUserData('user4', response.data)
        if (isPlayerTurn(response.data, 'user4')) {
          addLog('user4 turn.')
          if (userDatas.user4?.player_data?.hand[0]) {
            addLog(`user4 playing card ${CardName[userDatas.user4.player_data.hand[0].name]} on user1`)
            user4.playCard([userDatas.user4.player_data.hand[0]], ['user1'])
          }
        }
      },
      [ServerEventCode.LogicError]: (response) => {
        if (response.message) {
          addLog('user4 error: ' + response.message)
          if (response.message.includes('Targetted')) {
            addLog('looking for a missed...')

            const card = userDatas.user4?.player_data?.hand.find(card => card.name === CardName.Missed)
            if (card) {
              addLog('playing missed!')
              user4.playCard([card], [])
            } else {
              addLog('no missed.. taking damage.')
              user4.playCard([], [])
            }
          }
        }
      }
    })

    user1.connect('user1', {
      open: () => user2.connect('user2', {
        open: () => user3.connect('user3', {
          open: () => user4.connect('user4', {
            open: () => {
              addLog('successfully connected all users.')
              addLog('attempting to create a room...')
              user1.createSession()
            },
          }),
        }),
      }),
    })

    function isPlayerTurn(data: ServerEventData | undefined, player: string) {
      return data?.game_data?.player_order && data?.game_data?.player_order.indexOf(player) === data?.game_data?.turn_index
    }

    function disconnectAll() {
      user1.disconnect()
      user2.disconnect()
      user3.disconnect()
      user4.disconnect()
    }
  }, [])

  return (
    <div style={{ display: 'flex', flexDirection: 'row', margin: 'auto', width: '100%', height: '100%' }}>
      <Container>
        {logs.map((log, i) => <div key={i}>{log}</div>)}
      </Container>
      <Container>
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(4, 1fr)', gap: '1rem' }}>
          {
            Object.values(userDatas).map((userData, i) =>
              <div key={i} >
                user{i + 1}
                <pre style={{ maxHeight: '80vh', overflow: 'auto' }}>{JSON.stringify(userData, null, 2)}</pre>
              </div>
            )
          }
        </div>
      </Container>
    </div>
  )
}