import React from 'react'

import { setModalContextProvider } from '@/providers'

type Props = {
  children: React.ReactNode
}
export const Modal: React.FC<Props> = ({ children }) => {
  const modal = React.useContext(setModalContextProvider)
  return (
    <div>
      <button onClick={modal?.hideModal}>閉じる</button>
      {children}
    </div>
  )
}
