import create from 'zustand'
import { GameData } from '../utils/shared-types'
import { Accessor, Mutator } from './provider'


type GameDataStore = {
  data: GameData

  getCardEvents: Accessor<GameData['card_events']>
  getDiscard: Accessor<GameData['discard']>
  getPlayerOrder: Accessor<GameData['player_order']>
  getTurnIndex: Accessor<GameData['turn_index']>

  setCardEvents: Mutator<GameData['card_events']>
  setDiscard: Mutator<GameData['discard']>
  setPlayerOrder: Mutator<GameData['player_order']>
  setTurnIndex: Mutator<GameData['turn_index']>
}

export const useGameData = create<GameDataStore>((set, get) => ({
  data: {
    card_events: [],
    discard: [],
    player_order: [],
    turn_index: 0
  },

  getCardEvents: () => get().data.card_events,
  getDiscard: () => get().data.discard,
  getPlayerOrder: () => get().data.player_order,
  getTurnIndex: () => get().data.turn_index,

  setCardEvents: cardEvents => set(state => { state.data.card_events = cardEvents }),
  setDiscard: discard => set(state => { state.data.discard = discard }),
  setPlayerOrder: playerOrder => set(state => { state.data.player_order = playerOrder }),
  setTurnIndex: turnIndex => set(state => { state.data.turn_index = turnIndex }),
}))

