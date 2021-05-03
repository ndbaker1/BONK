import { IMessageEvent, w3cwebsocket as W3CWebSocket } from 'websocket'
import { environment } from '../environment'
import { ClientEvent, ClientEventCodes, ServerEvent, ServerEventCodes } from './event-types'


export class ClientConnection {
  private socket: W3CWebSocket | null = null
  private eventHandler: (event: IMessageEvent) => void

  constructor(callbacks: Record<ServerEventCodes, (response: ServerEvent) => void>) {
    this.eventHandler = this.create_event_handler(callbacks)
  }

  public connect(user_id: string, callbacks: Record<'open' | 'error', () => void>): void {
    const setupConnection = () => {
      this.socket = new W3CWebSocket(`${environment.wsProtocol}://${environment.apiDomain}/ws/${user_id}`)
      this.socket.onopen = () => {
        console.log('connected to websocket!', this.socket)
        callbacks.open()
      }
      this.socket.onmessage = this.eventHandler
      this.socket.onerror = (err) => {
        console.log('socket error!', err)
        callbacks.error()
      }
    }
    if (this.socket && this.socket.readyState != this.socket.CLOSED) {
      this.socket.onclose = () => {
        console.log('closed and reconnecting')
        setupConnection()
      }
      this.socket.close()
    } else {
      setupConnection()
    }
  }

  //=====================================
  // Receives Messages from the Server
  //=====================================
  private create_event_handler(callbacks: Record<number, (response: ServerEvent) => void>) {
    return (event: IMessageEvent) => {
      const response: ServerEvent = JSON.parse(event.data as string)
      switch (response.event_code) {
        case ServerEventCodes.ClientJoined: {
          callbacks[ServerEventCodes.ClientJoined](response)
          console.log('client', response.client_id, 'joined the room')
        } break
        case ServerEventCodes.ClientLeft: {
          callbacks[ServerEventCodes.ClientLeft](response)
          console.log('client', response.client_id, 'left the room')
        } break
        case ServerEventCodes.GameStarted: {
          callbacks[ServerEventCodes.GameStarted](response)
          console.log('game started')
        } break
        case ServerEventCodes.TurnStart: {
          callbacks[ServerEventCodes.TurnStart](response)
          console.log(`${response.client_id}'s turn has begun`)
        } break
        default:
          console.log(`unknown event_code: ${response.event_code}`)
      }
    }
  }

  //=====================
  // Public Methods
  //=====================
  public isOpen(): boolean {
    return !!this.socket && this.socket.readyState == this.socket.OPEN
  }

  public play_card(card_id: number): void {
    this.send_message({
      event_code: ClientEventCodes.PlayCard,
      card_id
    })
  }

  public create_session(): void {
    this.send_message({
      event_code: ClientEventCodes.CreateSession,
    })
  }

  public leave_session(): void {
    this.send_message({
      event_code: ClientEventCodes.LeaveSession,
    })
  }

  public join_session(session_id: string): void {
    this.send_message({
      event_code: ClientEventCodes.JoinSession,
      session_id: session_id
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
}
