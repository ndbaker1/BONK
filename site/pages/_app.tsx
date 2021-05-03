import { createMuiTheme } from '@material-ui/core'
import { ThemeProvider } from '@material-ui/styles'
import { AppProps } from 'next/app'
import '../styles/globals.css'

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

function MyApp({ Component, pageProps }: AppProps): JSX.Element {
  return (
    <ThemeProvider theme={theme}>
      <Component {...pageProps} />
    </ThemeProvider>
  )
}

export default MyApp
