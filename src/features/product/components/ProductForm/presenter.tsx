import React from 'react'

import { FormField } from '@/components'

type FormFieldAttributes = React.ComponentProps<typeof FormField>

type Props = Readonly<{
  mode?: 'create' | 'edit'
  formFieldAttributes: {
    name: FormFieldAttributes
    code: FormFieldAttributes
    unit: FormFieldAttributes
    defaultPrice: FormFieldAttributes
    standardStockQuantity: FormFieldAttributes
  }
  onSubmit: React.ComponentProps<'form'>['onSubmit']
}>
export const ProductFormPresenter: React.FC<Props> = ({ mode = 'create', formFieldAttributes, onSubmit }) => (
  <form onSubmit={onSubmit}>
    <FormField {...formFieldAttributes.name} />
    <FormField {...formFieldAttributes.code} />
    <FormField {...formFieldAttributes.unit} />
    <FormField {...formFieldAttributes.defaultPrice} />
    <FormField {...formFieldAttributes.standardStockQuantity} />
    <button type="submit">{mode === 'create' ? '登録' : '更新'}</button>
  </form>
)
