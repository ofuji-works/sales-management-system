import React from 'react'

import { Input } from '@/components/atoms'

type Props = Readonly<
  React.ComponentProps<typeof Input> & {
    label: string
    errorMessage?: string
  }
>
export const FormField: React.FC<Props> = React.forwardRef<HTMLInputElement, Props>(
  ({ label, errorMessage, ...attributes }, ref) => {
    return (
      <div>
        <label htmlFor={attributes.name}>{label}</label>
        <Input {...attributes} ref={ref} />
        <p>{errorMessage}</p>
      </div>
    )
  },
)
