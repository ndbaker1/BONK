
export type CardID = number

export type ServerEvent = {
   event_code: ServerEventCodes,
   session_id?: string,
   client_id?: string,
   session_client_ids?: Array<string>,
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
