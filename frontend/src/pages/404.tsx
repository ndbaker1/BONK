export default function NotFound(): JSX.Element | null {
  if (typeof window !== 'undefined') {
    window.location.replace('/')
  }
  return null
}