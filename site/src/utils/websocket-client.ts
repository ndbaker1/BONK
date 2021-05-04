import { IMessageEvent, w3cwebsocket as W3CWebSocket } from 'websocket'
import { environment } from '../environment'
import { ClientEvent, ClientEventCodes, ServerEvent, ServerEventCodes } from './event-types'

export class ClientConnection {
  private socket: W3CWebSocket | null = null
  private eventHandler: (event: IMessageEvent) => void

  constructor(callbacks: Record<ServerEventCodes, (response: ServerEvent) => void>) {
    this.eventHandler = this.create_event_handler(callbacks)
    // wake up app using the health endpoint
    // if (environment.healthCheck) {
    //   fetch(`${environment.http_or_https}://${environment.apiDomain}/health`).then(() => console.log('health check passed'))
    // }
  }

  public connect(user_id: string, callbacks: Record<'open' | 'error', () => void>): void {
    const setupConnection = () => {
      this.socket = new W3CWebSocket(`${environment.ws_or_wss}://${environment.apiDomain}/ws/${user_id}`)
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

  public disconnect(): void {
    this.socket?.close()
  }

  //=====================================
  // Receives Messages from the Server
  //=====================================
  private create_event_handler(callbacks: Record<number, (response: ServerEvent) => void>) {
    return (event: IMessageEvent) => {
      const response: ServerEvent = JSON.parse(event.data as string)
      callbacks[response.event_code](response)
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

  public join_session(session_id: string, errorCallback?: (err: string) => void): void {
    const error = this.verifySessionID(session_id)
    if (error) {
      errorCallback && errorCallback(error)
    } else {
      this.send_message({
        event_code: ClientEventCodes.JoinSession,
        session_id: session_id
      })
    }
  }

  public getState(): void {
    this.send_message({
      event_code: ClientEventCodes.DataRequest,
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
