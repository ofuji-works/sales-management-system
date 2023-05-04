import Link from 'next/link'

export const SideMenu = () => {
  return (
    <div>
      <ul>
        <li>
          <Link href="/product">商品マスター</Link>
        </li>
      </ul>
    </div>
  )
}
