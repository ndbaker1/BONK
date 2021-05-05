
export type CardID = number

export type ServerEvent = {
  event_code: ServerEventCodes,
  data?: ServerEventData,
}

export type ServerEventData = {
  session_id?: string,
  client_id?: string,
  session_client_ids?: Array<string>,
  game_state?: GameState,
}

export type GameState = {
  turn_index: number,
  turn_orders: Array<PlayerInfo>,
  player_blue_cards: Record<string, Array<BlueCards>>,
  player_green_cards: Record<string, Array<GreenCards>>,
  effect?: EffectCodes,
}

export enum EffectCodes {
  GeneralStore = 1,
}

export enum BlueCards {
  Barrel = 1,
  Dynamite,
}

export enum GreenCards {
  PonyExpress = 1,
}

export type PlayerInfo = {
  client_id: string,
  character_code: CharacterCodes,
}

export enum ServerEventCodes {
  // session_id, client_id, session_client_ids
  ClientJoined = 1,
  // client_id
  ClientLeft,
  GameStarted,
  // session_id, session_client_ids
  DataResponse,
  // session_id, client_id
  InvalidSessionID,
  // client_id
  TurnStart,
}

export type ClientEvent = {
  event_code: ClientEventCodes,
  target_ids?: Array<string>,
  card_id?: CardID,
  session_id?: string,
}

export enum ClientEventCodes {
  // session_id
  JoinSession = 1,
  CreateSession,
  LeaveSession,
  DataRequest,
  StartGame,
  EndTurn,
  PlayCard,
}

export enum CharacterCodes {
  Sheriff = 1,
  Renegade,
  Outlaw,
  Deputy,
}
