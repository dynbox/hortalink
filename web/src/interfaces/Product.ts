interface Product {
    id: number,
    product: {
        id: number,
        name: string
    },
    photos: string[],
    photo?: string,
    quantity: number,
    price: number,
    rating: number,
    rating_quantity: number,
    description: string,
    unit: string,
    unit_quantity: number

    seller_id: number
}

export type {
    Product
}