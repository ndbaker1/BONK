import React from 'react'
import { ThemeProvider } from '@material-ui/core/styles'
import { createMuiTheme } from '@material-ui/core'

const theme = createMuiTheme({
  palette: {
    primary: {
      main: '#00b8b8'
    },
    secondary: {
      main: '#00b8b8'
    },
  },
})

export default function ThemeWrapper({ element }: { element: JSX.Element }): JSX.Element {
  return (
    <ThemeProvider theme={theme}>
      {element}
    </ThemeProvider>
  )
}