
export type ServerEvent = {
  event_code: ServerEventCode,
  message?: string,
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
  player_hands: Record<string, Array<CardCode>>,
  player_fields: Record<string, Array<CardCode>>,
  effect?: EffectCode,
}

export enum EffectCode {
  GeneralStore = 1,
}

export enum CardCode {
  // Brown Cards
  Bang = 1,
  // Blue Cards
  Barrel,
  Dynamite,
  // Green Cards
  PonyExpress,
}

export type PlayerInfo = {
  client_id: string,
  character_code: CharacterCode,
}

export enum ServerEventCode {
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
  LogicError,
}

export type ClientEvent = {
  event_code: ClientEventCode,
  target_ids?: Array<string>,
  card_code?: CardCode,
  session_id?: string,
}

export enum ClientEventCode {
  // session_id
  JoinSession = 1,
  CreateSession,
  LeaveSession,
  DataRequest,
  StartGame,
  EndTurn,
  PlayCard,
}

export enum CharacterCode {
  Sheriff = 1,
  Renegade,
  Outlaw,
  Deputy,
}
