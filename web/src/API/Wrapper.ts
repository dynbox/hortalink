import type { User } from "@interfaces/User"
import RequestAPI from "./APIFunctions/RequestAPI"
import type { Product, ProductFilter, ProductFullTextSearch } from "@interfaces/Product"

class APIWrapper<F extends RequestAPIFrom> {
    private from: F

    constructor(from: F) {
        this.from = from
    }

    async getCurrentSession(session_id?: F extends RequestAPIFrom.Server ? string : never): Promise<User> {
        switch(this.from) {
            case RequestAPIFrom.Client:
                return this.getCurrentSessionFromClient()
            case RequestAPIFrom.Server:
                return this.getCurrentSessionFromServer(session_id)
        }
    }

    private async getCurrentSessionFromClient(): Promise<User> {
        const data = await RequestAPI(this.from, "/v1/users/@me", null, "include") as User
        return data
    }

    private async getCurrentSessionFromServer(session_id: string): Promise<User> {
        const data = await RequestAPI(this.from, "/v1/users/@me", null, "include", {
            'Cookie': `session_id=${session_id}`
        }) as User
        return data
    }

    public async getRecentProducts(page: number = 1): Promise<Product[]> {
        const searchParams = new URLSearchParams()

        searchParams.append("page", String(page))
        searchParams.append("per_page", String(10))

        const data = await RequestAPI(this.from, "/v1/users/@me/home/most_recent", searchParams, "include") as Product[]
        return data
    }

    public async getMoreOrderProducts(page: number = 1): Promise<Product[]> {
        const searchParams = new URLSearchParams()

        searchParams.append("page", String(page))
        searchParams.append("per_page", String(10))

        const data = await RequestAPI(this.from, "/v1/users/@me/home/more_orders", searchParams, "include") as Product[]
        return data
    }

    public async getProducts(filter: ProductFilter): Promise<Product[]> {
        const searchParams = new URLSearchParams()

        if(!filter.page) {
            filter.page = 1
        }

        if(!filter.per_page) {
            filter.per_page = 10
        }

        const keys = Object.keys(filter)

        for(const key of keys) {
            const value = filter[key]

            searchParams.append(key, value)
        }

        const data = await RequestAPI(this.from, "/v1/products", searchParams, "include") as Product[]
        return data
    }
    
    public async search(query: string, page: number = 1, per_page: number = 10): Promise<ProductFullTextSearch[]> {
        const searchParams = new URLSearchParams()
    
        searchParams.append("query", query)
        searchParams.append("page", String(page))
        searchParams.append("per_page", String(per_page))
    
        const types = await RequestAPI(this.from, "/v1/resources/products", searchParams, "include") as ProductFullTextSearch[]
    
        return types
    }
}


enum RequestAPIFrom {
    Server = 1,
    Client = 2
}

export {
    RequestAPIFrom
}

export default APIWrapper