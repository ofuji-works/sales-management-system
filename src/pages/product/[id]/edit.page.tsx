import { type NextPage } from 'next'
import Link from 'next/link'
import { useRouter } from 'next/router'
import React, { useEffect, useState } from 'react'

import { Product, findByIdProduct, updateProduct } from '@/api/product'
import { ProductForm, isPageParameters } from '@/features/product'

const Page: NextPage = () => {
  const router = useRouter()
  const [loading, setLoading] = useState<boolean>(false)
  const [product, setProduct] = useState<Product>()

  useEffect(() => {
    if (isPageParameters(router.query)) {
      const id = parseInt(router.query.id, 10)
      setLoading(true)
      findByIdProduct(id)
        .then((response) => {
          setProduct(response.product)
        })
        .finally(() => {
          setLoading(false)
        })
    }
  }, [])

  const submitHandler: React.ComponentProps<typeof ProductForm>['onSubmit'] = (data) => {
    if (isPageParameters(router.query)) {
      const params = {
        id: parseInt(router.query.id, 10),
        name: data.name,
        code: data.code,
        unit: data.unit,
        default_price: data.defaultPrice,
        standard_stock_quantity: data.standardStockQuantity,
      }
      updateProduct(params)
    }
  }

  if (loading) {
    return <p>now loading ...</p>
  }

  return (
    <div>
      <Link href={`/product/${router.query.id}`}>詳細画面に戻る</Link>
      <ProductForm
        onSubmit={submitHandler}
        defaultValues={{
          name: product?.name,
          code: product?.code,
          unit: product?.unit,
          defaultPrice: product?.default_price,
          standardStockQuantity: product?.standard_stock_quantity,
        }}
      />
    </div>
  )
}

export default Page
