import Products from "@layouts/Products";
import Product from "@components/Product";
import { useEffect, useState } from "react";
import type { Product as ProductT } from "@interfaces/Product";

import APIWrapper, { RequestAPIFrom } from "@HortalinkAPIWrapper";
import { filter } from "../Search";

import ProductsColumn from "@layouts/ProductsColumn";
import ProductColumn from "@components/ProductColumn";

export default function Menu() {
    const api = new APIWrapper(RequestAPIFrom.Client)
    const [products, setProducts] = useState<ProductT[]>()

    useEffect(() => {
        async function wrap() {
            const data = await api.getProducts(filter.get() || { page: 1, per_page: 10 })
            setProducts(data)
        }

        wrap()
    }, [])

    return (
        <div className="search_products_container">
            <Products>
                {
                    products?.map((product) => (
                        <Product product={product} key={`menu-product-${product.id}`} />
                    ))
                }
            </Products>
        </div>
    )
}