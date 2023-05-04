import * as z from 'zod'

export const schema = z.object({
  name: z.string().min(1, '商品名を入力してください'),
  code: z.string().min(1, '商品コードを入力してください'),
  unit: z.string().min(1, '商品の単位を入力してください'),
  defaultPrice: z.number({ required_error: 'デフォルトの価格を入力してください' }),
  standardStockQuantity: z.number({ required_error: '標準在庫数を入力してください' }),
})
