import { IMessageEvent, w3cwebsocket as W3CWebSocket } from 'websocket'
import { environment } from '../environment'
import { ClientEvent, ClientEventCodes, ServerEvent, ServerEventCodes } from './event-types'

export class ClientConnection {
  private socket: W3CWebSocket | null = null

  public connect(user_id: string, callback: () => void): void {
    this.socket && this.socket.close()
    this.socket = new W3CWebSocket(`ws://${environment.apiDomain}/ws/${user_id}`)
    this.socket.onopen = () => {
      console.log('connected to websocket!')
      callback()
    }
    this.socket.onmessage = this.event_handler
    this.socket.onclose = () => console.log('socket close!')
    this.socket.onerror = () => console.log('socket error!')
  }

  //=====================================
  // Receives Messages from the Server
  //=====================================
  private event_handler(event: IMessageEvent) {
    const response: ServerEvent = JSON.parse(event.data as string)
    switch (response.event_code) {
      case ServerEventCodes.SessionCreated: {
        console.log('loading into new room:', response.session_id)
      } break
      case ServerEventCodes.ClientJoined: {
        console.log('client', response.client_id, 'joined the room')
      } break
      case ServerEventCodes.ClientLeft: {
        console.log('client', response.client_id, 'left the room')
      } break
      default:
        console.log(`unknown event_code: ${response.event_code}`)
    }
  }

  //=====================
  // Public Methods
  //=====================
  public is_open(): boolean {
    return this.socket?.readyState == this.socket?.OPEN
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
    this.socket?.send(JSON.stringify(session_update))
  }
}
