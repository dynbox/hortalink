import request_api from "./request_api";

export default async function SearchTypes(query: string, page: number = 1, per_page: number = 10) {
    const url = new URL("http://localhost/api/v1/resources/products")

    url.searchParams.append("query", query)
    url.searchParams.append("page", String(page))
    url.searchParams.append("per_page", String(per_page))

    const types = await request_api(url.pathname + url.search)

    return types
}