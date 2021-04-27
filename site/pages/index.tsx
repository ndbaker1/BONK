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
        client.connect(1)
      }}> Connect </button>
    </div>
  )
}
