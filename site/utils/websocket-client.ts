import { IMessageEvent, w3cwebsocket as W3CWebSocket } from 'websocket'
import { environment } from '../environment'

export class ClientConnection {
  private socket: W3CWebSocket | null = null

  public connect(user_id: string): void {
    this.socket && this.socket.close()
    this.socket = new W3CWebSocket(`ws://${environment.apiDomain}/ws/${user_id}`)
    this.socket.onopen = () => this.init_state()
    this.socket.onmessage = this.event_handler
  }

  public is_open(): boolean {
    return this.socket?.readyState == this.socket?.OPEN
  }

  private init_state() {
    console.log('connected to websocket!')
    this.send_update({ event_code: 1 })
  }

  private event_handler(event: IMessageEvent) {
    console.log('event:', event.data)
  }

  private send_update(session_update: any) {
    this.socket?.send(JSON.stringify(session_update))
  }

  public send_card(event_code: number): void {
    this.socket?.send(JSON.stringify({
      event_code,
      target_ids: [],
      card_id: 2,
    }))
  }

  public create_session(): void {
    this.socket?.send('create_session')
  }
}