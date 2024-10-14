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

interface ProductFilter {
    max_price?: number
    min_price?: number
    min_stars?: number
    product_type?: number
    start_time?: string
    product_id?: number
    day_of_week?: number
    page: number
    per_page: number
    latitude?: string
    longitude?: string,
    distance?: number
}

interface ProductFullTextSearch {
    product_id: number,
    product_name: string,
    alias: string[],
    category_name: string,
    category_id: number
}

export type {
    Product,
    ProductFilter,
    ProductFullTextSearch
}