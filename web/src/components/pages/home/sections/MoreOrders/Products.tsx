import type { Product as ProductT } from "@interfaces/Product";
import Products from "@layouts/Products";
import { useEffect, useState } from "react";

import Product from "@components/Product";
import APIWrapper, { RequestAPIFrom } from "@HortalinkAPIWrapper";

export default function RecentProducts() {
    const api = new APIWrapper(RequestAPIFrom.Client)
    const [products, setProducts] = useState<ProductT[]>([])
    const [page, setPage] = useState<number>(1)

    useEffect(() => {
        async function run() {
            const data = await api.getRecentProducts(page)

            setProducts(data)
        }
        
        run()
    }, [])

    return (
        <Products>
            {
                products.map((product, i) => {
                    return (
                        <Product product={product} key={`recent-${product.id}`} />
                    )
                })
            }
        </Products>
    )
}