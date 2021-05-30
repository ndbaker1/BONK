import create from 'zustand'
import { Mutator } from './provider'

type Notification = string

type NotificationStore = {
  notification: Notification
  setNotification: Mutator<Notification>
}

export const useNotification = create<NotificationStore>(set => ({
  notification: '',
  setNotification: notification => set({ notification }),
}))

