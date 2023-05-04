import Link from 'next/link'
import React from 'react'

import { createProduct } from '@/api/product'
import { ProductForm } from '@/features/product'

export default () => {
  const submitHandler: React.ComponentProps<typeof ProductForm>['onSubmit'] = (data) => {
    createProduct({
      name: data.name,
      code: data.code,
      unit: data.unit,
      default_price: data.defaultPrice,
      standard_stock_quantity: data.standardStockQuantity,
    })
  }
  return (
    <div>
      <Link href="/product">一覧に戻る</Link>
      <ProductForm onSubmit={submitHandler} />
    </div>
  )
}
