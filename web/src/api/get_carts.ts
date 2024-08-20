import request_api from "./request_api";

export default async function get_carts(session_id?: string) {
    console.log(session_id)
    return await request_api("/api/v1/users/@me/cart", "include", `session_id=${session_id}`)
}