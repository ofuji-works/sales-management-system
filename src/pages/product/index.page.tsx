import { type NextPage } from 'next'
import Link from 'next/link'
import React from 'react'

import { deleteProduct, searchProduct, type Product } from '@/api/product'
import { setModalContextProvider } from '@/providers'

const initialPageMetaData = {
  limit: 10,
  offset: 0,
}

const Page: NextPage = () => {
  const [loading, setLoading] = React.useState<boolean>(false)
  const [products, setProducts] = React.useState<Product[]>([])
  const modal = React.useContext(setModalContextProvider)

  React.useEffect(() => {
    setLoading(true)
    searchProduct(initialPageMetaData)
      .then((response) => {
        setProducts(response.products)
      })
      .finally(() => {
        setLoading(false)
      })
  }, [])

  const openDeleteModal = (productId: Product['id']) => {
    const deleteHandler = () => {
      deleteProduct({ product_id: productId })
    }

    console.debug(modal)

    modal?.openModal(
      <div>
        <button onClick={deleteHandler}></button>
      </div>,
    )
  }

  if (loading) {
    return <p>now loading ...</p>
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
                <button onClick={() => openDeleteModal(product.id)}>削除</button>
              </li>
            )
          })}
        </ul>
      </div>
    </div>
  )
}

export default Page
