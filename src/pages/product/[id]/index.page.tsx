import { type NextPage } from 'next'
import Link from 'next/link'
import { useRouter } from 'next/router'
import React from 'react'

import { Product, deleteProduct, findByIdProduct } from '@/api/product'
import { isPageParameters } from '@/features/product'
import { setModalContextProvider } from '@/providers'

const Page: NextPage = () => {
  const router = useRouter()
  const [loading, setLoading] = React.useState<boolean>(false)
  const [product, setProduct] = React.useState<Product>()
  const modal = React.useContext(setModalContextProvider)

  React.useEffect(() => {
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

  const openDeleteModal = () => {
    if (product) {
      const deleteHandler = () => {
        deleteProduct({ product_id: product.id }).then(() => {
          router.push('/product')
        })
      }

      modal?.openModal(
        <div>
          <button onClick={deleteHandler}></button>
        </div>,
      )
    }
  }

  if (loading) {
    return <p>now loading ...</p>
  }

  return (
    <div>
      <Link href="/product">一覧に戻る</Link>
      <Link href={`/product/${router.query.id}/edit`}>編集する</Link>
      <button onClick={openDeleteModal}>削除する</button>
      <dl>
        <dt>商品名</dt>
        <dd>{product?.name}</dd>
      </dl>
      <dl>
        <dt>商品コード</dt>
        <dd>{product?.code}</dd>
      </dl>
      <dl>
        <dt>単位</dt>
        <dd>{product?.unit}</dd>
      </dl>
      <dl>
        <dt>デフォルト価格</dt>
        <dd>{product?.default_price}</dd>
      </dl>
      <dl>
        <dt>標準在庫数</dt>
        <dd>{product?.standard_stock_quantity}</dd>
      </dl>
    </div>
  )
}

export default Page
