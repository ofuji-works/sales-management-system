import { invoke } from '@/utils/tauri'

export type Product = {
  id: number
  name: string
  code: string
  unit: string
  default_price: number
  standard_stock_quantity: number
  created_at: string
  updated_at: string
  deleted_at?: string
}

type ProductSearchParameters = {
  name?: string
  code?: string
  offset?: number
  limit?: number
}

type FindByIdProductResponse = {
  product: Product
}

export const findByIdProduct = (productId: number) => {
  return invoke<FindByIdProductResponse>('find_by_id_product', {
    request: {
      product_id: productId,
    },
  })
}

type ProductSearchResponse = {
  products: Product[]
}

export const searchProduct = (params: ProductSearchParameters) => {
  return invoke<ProductSearchResponse>('search_product', {
    request: params,
  })
}

type ProductCreateParameters = {
  name: Product['name']
  code: Product['code']
  unit: Product['unit']
  default_price: Product['default_price']
  standard_stock_quantity: Product['standard_stock_quantity']
}

export const createProduct = (params: ProductCreateParameters) => {
  return invoke('create_product', {
    request: params,
  })
}

type ProductUpdateParameters = {
  id: Product['id']
  name?: Product['name']
  code?: Product['code']
  default_price?: Product['default_price']
  standard_stock_quantity?: Product['standard_stock_quantity']
}

export const updateProduct = (params: ProductUpdateParameters) => {
  return invoke('update_product', {
    request: params,
  })
}
