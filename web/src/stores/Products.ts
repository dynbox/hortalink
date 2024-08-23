import { atom } from "nanostores";
import type { Rating } from "../interfaces/Rating";
import type { Product } from "../interfaces/Product";

export default {
    recent: atom([]),
    most_requested: atom([]),
    searched: atom([]),
    search_result: atom([]),
    current_product: atom<Product>(),
    current_product_quantity: atom(0),
    current_product_ratings: atom<Rating>(),
    all_products_dist: atom<{ [key: number]: number }>(),

    products: atom<Product[]>([]) // for selected products, ex: seller specific products.
}