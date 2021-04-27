import { IMessageEvent, w3cwebsocket as W3CWebSocket } from 'websocket'
import { environment } from '../environment'

export class ClientConnection {
  private socket: W3CWebSocket | null = null

  public connect(user_id: number): void {
    // register on the endpoint and connect to websocket
    fetch('http://' + environment.apiDomain + '/register', {
      method: 'POST',
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        user_id
      })
    }).then(res => res.json())
      .then((res) => {
        this.socket && this.socket.close()
        this.socket = new W3CWebSocket(res.url)
        this.socket.onopen = this.initState
        this.socket.onmessage = this.eventHandler
      })
  }


  private initState() {
    console.log('connected to websocket!')
    this.sendUpdate(['update'])
  }

  private eventHandler(event: IMessageEvent) {
    console.log('event:', event.data)
  }


  public sendUpdate(topics: string[]): void {
    if (this.socket) {
      this.socket.send(JSON.stringify({ topics }))
    }
  }
}