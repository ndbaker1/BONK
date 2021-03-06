/**
 * This file contains type defintions which are shared between the front and back end applications
 */

export type ServerEventData = {
  session_id?: string,
  client_id?: string,
  session_client_ids?: Array<string>,
  game_data?: GameData,
  player_data?: PlayerData,
}

export type PlayerData = {
  health: number,
  hand: Array<Card>,
  field: Array<Card>,
  character: Character,
  role: Role,
}

export type GameData = {
  turn_index: number,
  player_order: Array<string>,
  card_events: Array<CardName>,
  discard: Array<Card>,
}

export type ServerEvent = {
  event_code: ServerEventCode,
  message?: string,
  data?: ServerEventData,
}

export enum EffectCode {
  GeneralStore = 1,
  None,
}

export type Card = {
  name: CardName,
  suit: CardSuit,
  rank: CardRank,
}

export enum CardName {
  // Brown Cards
  Bang = 1,
  Hatchet,
  Missed,
  // Blue Cards
  Barrel,
  Dynamite,
  // Green Cards
  PonyExpress,
}

export enum CardSuit {
  Clubs = 1,
  Diamonds,
  Hearts,
  Spades,
}

export enum CardRank {
  N1 = 1,
  N2,
  N3,
  N4,
  N5,
  N6,
  N7,
  N8,
  N9,
  N10,
  J,
  Q,
  K,
  A,
}

export enum ServerEventCode {
  // session_id, client_id, session_client_ids
  ClientJoined = 1,
  // client_id
  ClientLeft,
  GameStarted,
  // session_id, session_client_ids
  DataResponse,
  // client_id
  TurnStart,
  LogicError,
}

export type ClientEvent = {
  event_code: ClientEventCode,
  target_ids?: Array<string>,
  cards?: Array<Card>,
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
  StateResponse,
}

export enum Role {
  Sheriff = 1,
  Renegade,
  Outlaw,
  Deputy,
}

export enum Character {
  BillyTheKid = 1,
}

export type ResponseData = {
  cards: Array<CardName>,
  characters: Array<Character>,
}
