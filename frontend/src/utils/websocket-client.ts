import { IMessageEvent, w3cwebsocket as W3CWebSocket } from 'websocket'
import { environment } from '../environment'
import { Card, ClientEvent, ClientEventCode, ServerEvent, ServerEventCode } from './shared-types'


export class ServerConnection {
  private socket: W3CWebSocket | null = null
  private eventHandler: (event: IMessageEvent) => void

  constructor(callbacks: Record<ServerEventCode, (response: ServerEvent) => void>) {
    this.eventHandler = this.create_event_handler(callbacks)
  }

  public connect(user_id: string, callbacks: {
    open: () => void
    close: () => void
    error: (err: unknown) => void
  }): void {
    const setupConnection = () => {
      this.socket = new W3CWebSocket(`${environment.ws_or_wss}://${environment.apiDomain}/ws/${user_id}`)
      this.socket.onmessage = this.eventHandler
      this.socket.onopen = () => callbacks.open()
      this.socket.onclose = () => callbacks.close()
      this.socket.onerror = err => callbacks.error(err)
    }
    if (this.socket && this.socket.readyState != this.socket.CLOSED) {
      this.socket.onclose = () => {
        callbacks.close()
        setupConnection()
      }
      this.socket.close()
    } else {
      setupConnection()
    }
  }

  public disconnect(): void {
    this.socket?.close()
  }

  //=====================================
  // Receives Messages from the Server
  //=====================================
  private create_event_handler(callbacks: Record<number, (response: ServerEvent) => void>) {
    return (event: IMessageEvent) => {
      const response: ServerEvent = JSON.parse(event.data as string)
      // console.log('event handler:', response)
      callbacks[response.event_code](response)
    }
  }

  //=====================
  // Public Methods
  //=====================
  public isOpen(): boolean {
    return !!this.socket && this.socket.readyState == this.socket.OPEN
  }

  public play_card(cards: Card[], targets: string[]): void {
    this.send_message({
      event_code: ClientEventCode.PlayCard,
      target_ids: targets,
      cards,
    })
  }

  public create_session(): void {
    this.send_message({
      event_code: ClientEventCode.CreateSession,
    })
  }

  public leave_session(): void {
    this.send_message({
      event_code: ClientEventCode.LeaveSession,
    })
  }

  public startGame(): void {
    this.send_message({
      event_code: ClientEventCode.StartGame,
    })
  }


  public join_session(session_id: string, errorCallback?: (err: string) => void): void {
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

  public fetchSession(): void {
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

  private verifySessionID(sessionID: string): string {
    const sessionIDLength = 5
    const errors: string[] = []
    if (sessionID.length !== sessionIDLength) {
      errors.push(`SessionID needs to be ${sessionIDLength} characters`)
    }
    return errors.join('')
  }
}
