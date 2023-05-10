import React from 'react'

import { Modal } from '@/components/organisms'

const modalContextProvider = React.createContext<React.ReactNode>(null)
export const setModalContextProvider = React.createContext<null | {
  openModal: (node: React.ReactNode) => void
  hideModal: () => void
}>(null)

type Props = {
  children: React.ReactNode
}
export const ModalProvider: React.FC<Props> = ({ children }) => {
  const [content, setContent] = React.useState<React.ReactNode>(null)
  const openModal = (node: React.ReactNode) => {
    setContent(node)
  }
  const hideModal = () => {
    setContent(null)
  }
  return (
    <setModalContextProvider.Provider
      value={{
        openModal,
        hideModal,
      }}
    >
      <modalContextProvider.Provider value={content}>
        {children}
        {content && <Modal>{content}</Modal>}
      </modalContextProvider.Provider>
    </setModalContextProvider.Provider>
  )
}
