import type { Cart } from "./Cart";
import type { CartSeller } from "./RawCart";

interface CartGroup {
    seller: CartSeller,
    carts: Cart[]
}

export type {
    CartGroup
}