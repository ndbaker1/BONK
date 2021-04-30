export type CardID = number

export type ClientEvent = {
  event_code: ClientEventCodes,
  target_ids?: string[],
  card_id?: CardID,
  session_id?: string,
}

export enum ClientEventCodes {
  JoinSession = 1,
  CreateSession,
  LeaveSession,
  PlayCard,
}

export type ServerEvent = {
  event_code: ServerEventCodes,
  session_id?: string,
  client_id?: string,
}

export enum ServerEventCodes {
  SessionCreated = 1,
  ClientJoined,
  ClientLeft,
}
