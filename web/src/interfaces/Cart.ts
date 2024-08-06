import type { CartProduct, CartSeller } from "./RawCart"

interface CartProductWithAmount extends CartProduct {
    amount: number
}

interface Cart {
    id: number,
    seller: CartSeller,
    amount: number,
    withdrawn: number,
    product: CartProductWithAmount[]
}

export type {
    Cart
}