import Head from 'next/head'
import { ClientConnection } from '../utils/websocket-client'

export default function Home(): JSX.Element {
  const client = new ClientConnection()

  return (
    <div>
      <Head>
        <title>Create Next App</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <button onClick={() => {
        client.connect('test_user')
      }}> Connect </button>
      <button onClick={() => {
        client.send_card(2)
      }}> Card 2 </button>
      <button onClick={() => {
        client.create_session()
      }}> Create Session </button>
    </div>
  )
}
