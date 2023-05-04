import React from 'react'
import { useForm } from 'react-hook-form'
import { type infer as Infer } from 'zod'
import { zodResolver } from '@hookform/resolvers/zod'

import { ProductFormPresenter } from './presenter'

import { schema } from '@/features/product/schema'

type FormData = Infer<typeof schema>

type Props = {
  onSubmit: (formData: FormData) => void
  defaultValues?: Partial<FormData>
}
export const ProductForm: React.FC<Props> = ({ onSubmit, defaultValues }) => {
  const { register, handleSubmit, formState } = useForm<FormData>({
    resolver: zodResolver(schema),
    mode: 'all',
    reValidateMode: 'onChange',
    defaultValues,
  })

  const submitHandler = (data: Readonly<FormData>) => {
    onSubmit(data)
  }

  const formFieldAttributes = {
    name: {
      ...register('name'),
      label: '商品名',
      errorMessage: formState.errors.name?.message,
    },
    code: {
      ...register('code'),
      label: '商品コード',
      errorMessage: formState.errors.code?.message,
    },
    unit: {
      ...register('unit'),
      label: '単位',
      errorMessage: formState.errors.unit?.message,
    },
    defaultPrice: {
      ...register('defaultPrice', { valueAsNumber: true }),
      label: 'デフォルト価格',
      type: 'number',
      errorMessage: formState.errors.defaultPrice?.message,
    },
    standardStockQuantity: {
      ...register('standardStockQuantity', { valueAsNumber: true }),
      label: '標準在庫数',
      type: 'number',
      errorMessage: formState.errors.standardStockQuantity?.message,
    },
  }

  return <ProductFormPresenter onSubmit={handleSubmit(submitHandler)} formFieldAttributes={formFieldAttributes} />
}
