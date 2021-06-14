import create from 'zustand'
import { GameData } from '../utils/shared-types'
import { Accessor, Mutator } from './provider'


type GameDataStore = {
  data: GameData
  // & { logs: string[] }

  getCardEvents: Accessor<GameData['round']>
  getDiscard: Accessor<GameData['discard']>
  getPlayerOrder: Accessor<GameData['player_order']>
  getTurnIndex: Accessor<GameData['turn_index']>

  setCardEvents: Mutator<GameData['round']>
  setDiscard: Mutator<GameData['discard']>
  setPlayerOrder: Mutator<GameData['player_order']>
  setTurnIndex: Mutator<GameData['turn_index']>

  // getLogs: Accessor<string[]>
  // log: Mutator<string>
}

export const useGameData = create<GameDataStore>((set, get) => ({
  data: {
    round: 0,
    discard: [],
    player_order: [],
    turn_index: 0,
    // logs: []
  },

  getCardEvents: () => get().data.round,
  getDiscard: () => get().data.discard,
  getPlayerOrder: () => get().data.player_order,
  getTurnIndex: () => get().data.turn_index,

  setCardEvents: cardEvents => set(state => { state.data.round = cardEvents }),
  setDiscard: discard => set(state => { state.data.discard = discard }),
  setPlayerOrder: playerOrder => set(state => { state.data.player_order = playerOrder }),
  setTurnIndex: turnIndex => set(state => { state.data.turn_index = turnIndex }),


  // getLogs: () => get().data.logs,
  // log: log => set(state => {
  //   console.log(log)
  //   state.data.logs = [...state.data.logs, log]
  // }),
}))

