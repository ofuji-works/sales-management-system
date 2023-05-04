import React from 'react'

type Props = React.ComponentProps<'input'>
export const Input = React.forwardRef<HTMLInputElement, Props>((props, ref) => {
  return <input ref={ref} {...props} />
})
