
export type CardID = number

export type ServerEvent = {
  event_code: ServerEventCodes,
  session_id?: string,
  client_id?: string,
}

export enum ServerEventCodes {
  ClientJoined = 1,
  ClientLeft,
  GameStarted,
  TurnStart,
}

export type ClientEvent = {
  event_code: ClientEventCodes,
  target_ids?: Array<string>,
  card_id?: CardID,
  session_id?: string,
}

export enum ClientEventCodes {
  JoinSession = 1,
  CreateSession,
  LeaveSession,
  StartGame,
  EndTurn,
  PlayCard,
}
