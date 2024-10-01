import type { User } from "@interfaces/User"
import RequestAPI from "./APIFunctions/RequestAPI"
import type { Product } from "@interfaces/Product"

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
}


enum RequestAPIFrom {
    Server = 1,
    Client = 2
}

export {
    RequestAPIFrom
}

export default APIWrapper