import type { ProductResume } from "./Product";

interface UserProfile {
    name: string,
    id: number,
    avatar?: string,
    is_seller: boolean,
    followers: number,
    roles: number[],
    orders_received: number
}

interface User {
    profile: UserProfile,
    products: ProductResume[]
}

export type {
    User,
    UserProfile
}