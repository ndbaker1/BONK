import { Subject } from 'rxjs'
import { IMessageEvent, w3cwebsocket as W3CWebSocket } from 'websocket'
import { environment } from '../environment'
import { Card, Character, ClientEvent, ClientEventCode, ClientEventIntent, ServerEvent, ServerEventCode } from './shared-types'

const emptyCallback = () => 0

type ServerEventCallbacks = Record<ServerEventCode, Subject<ServerEvent>>

export class ServerConnection {
  private socket: W3CWebSocket | null = null
  private callbacks: ServerEventCallbacks = {
    [ServerEventCode.ClientJoined]: new Subject(),
    [ServerEventCode.ClientLeft]: new Subject(),
    [ServerEventCode.GameStarted]: new Subject(),
    [ServerEventCode.Damage]: new Subject(),
    [ServerEventCode.DataResponse]: new Subject(),
    [ServerEventCode.Draw]: new Subject(),
    [ServerEventCode.Action]: new Subject(),
    [ServerEventCode.LogicError]: new Subject(),
    [ServerEventCode.Targetted]: new Subject(),
    [ServerEventCode.TurnStart]: new Subject(),
  }

  addHandler(eventCode: ServerEventCode, handler: (response: ServerEvent) => void): void {
    this.callbacks[eventCode].subscribe(handler)
  }

  connect(user_id: string, callbacks: {
    open?: () => void
    close?: () => void
    error?: (err: unknown) => void
  }): void {
    const setupConnection = () => {
      this.socket = new W3CWebSocket(`${environment.ws_or_wss}://${environment.apiDomain}/ws/${user_id}`)
      this.socket.onmessage = this.eventHandler
      this.socket.onopen = () => (callbacks.open ?? emptyCallback)()
      this.socket.onclose = () => (callbacks.close ?? emptyCallback)()
      this.socket.onerror = err => (callbacks.error ?? emptyCallback)(err)
    }
    if (this.socket && this.socket.readyState != this.socket.CLOSED) {
      this.socket.onclose = () => {
        (callbacks.close ?? emptyCallback)()
        setupConnection()
      }
      this.socket.close()
    } else {
      setupConnection()
    }
  }

  disconnect(): void {
    this.socket?.close()
  }

  //=====================================
  // Receives Messages from the Server
  //=====================================
  private eventHandler = (event: IMessageEvent) => {
    const response: ServerEvent = JSON.parse(event.data as string)
    console.log('event handler:', response)
    this.callbacks[response.event_code].next(response)
  }

  //=====================
  // Public Methods
  //=====================
  isOpen(): boolean {
    return !!this.socket && this.socket.readyState == this.socket.OPEN
  }

  playCard(cards: Card[], targets: string[], intent?: ClientEventIntent): void {
    this.send_message({
      event_code: ClientEventCode.PlayerAction,
      intent,
      target_ids: targets,
      cards,
    })
  }

  useAbility(character: Character, targets: string[], intent?: ClientEventIntent): void {
    this.send_message({
      event_code: ClientEventCode.PlayerAction,
      intent,
      target_ids: targets,
      character,
    })
  }

  createSession(): void {
    this.send_message({
      event_code: ClientEventCode.CreateSession,
    })
  }

  leaveSession(): void {
    this.send_message({
      event_code: ClientEventCode.LeaveSession,
    })
  }

  startGame(): void {
    this.send_message({
      event_code: ClientEventCode.StartGame,
    })
  }

  joinSession(session_id: string, errorCallback?: (err: string) => void): void {
    const error = this.verifySessionID(session_id)
    if (error) {
      errorCallback && errorCallback(error)
    } else {
      this.send_message({
        event_code: ClientEventCode.JoinSession,
        session_id: session_id
      })
    }
  }

  fetchState(): void {
    this.send_message({
      event_code: ClientEventCode.DataRequest,
    })
  }

  //======================================
  // Sends Client Messages to the Server
  //======================================
  private send_message(session_update: ClientEvent) {
    if (!this.socket)
      console.log('socket not connected!')
    else
      this.socket.send(JSON.stringify(session_update))
  }

  //========================
  // Utility Functions
  //========================
  private verifySessionID(sessionID: string): string {
    const sessionIDLength = 5
    const errors: string[] = []
    if (sessionID.length !== sessionIDLength) {
      errors.push(`SessionID needs to be ${sessionIDLength} characters`)
    }
    return errors.join('')
  }
}
