import { Paper } from '@material-ui/core'
import React from 'react'

export default function Container(props: React.PropsWithChildren<unknown>): JSX.Element {
  return (
    <Paper elevation={7} style={{ display: 'flex', flexDirection: 'column', margin: 'auto', padding: '2rem' }}>
      <div style={{ display: 'flex', flexDirection: 'column', margin: 'auto' }}>
        {props.children}
      </div>
    </Paper>
  )
}