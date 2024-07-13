export default async function request_api(path: string) {
    const request = await fetch(`http://localhost:5443${path}`, {
        credentials: "include"
    })

    if(request.ok) {
        return await request.json()
    } else {
        throw new Error("Request error")
    }
}