interface Filter {
    max_price?: string
    min_price?: string
    min_stars?: string
    product_type?: string
    start_time?: string
    product_id?: string
    day_of_week?: string
    page: number;
    per_page: number;
    latitude?: string
    longitude?: string
}

export type {
    Filter
}