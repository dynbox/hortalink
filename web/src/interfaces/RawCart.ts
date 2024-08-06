interface RawCart {
    id: number,
    seller: CartSeller,
    amount: number,
    withdrawn: number,
    product: CartProduct
}

interface CartSeller {
    id: number,
    name: string,
    avatar?: string
}

interface CartProduct {
    id: number,
    name: string,
    price: number,
    photos?: string[],
    unit: string,
    dist?: number
}

export type {
    RawCart,
    CartSeller,
    CartProduct
}