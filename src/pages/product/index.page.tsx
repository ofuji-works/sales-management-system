import Link from 'next/link'
import { useEffect, useState } from 'react'

import { searchProduct, type Product } from '@/api/product'

const initialPageMetaData = {
  limit: 10,
  offset: 0,
}

const Page = () => {
  const [loading, setLoading] = useState<boolean>(false)
  const [products, setProducts] = useState<Product[]>([])

  useEffect(() => {
    setLoading(true)
    searchProduct(initialPageMetaData)
      .then((response) => {
        setProducts(response.products)
      })
      .finally(() => {
        setLoading(false)
      })
  }, [])

  if (loading) {
    return 'now loading ...'
  }

  return (
    <div>
      <h1>商品マスター</h1>
      <Link href="product/create">新規作成</Link>
      <div>
        <ul>
          {products.map((product) => {
            return (
              <li key={product.code}>
                {product.name}
                <Link href={`/product/${product.id}`}>product</Link>
              </li>
            )
          })}
        </ul>
      </div>
    </div>
  )
}

export default Page
