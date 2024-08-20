export default async function request_api(path: string, credentials: RequestCredentials = "include", cookie?: string) {
    const request = await fetch(`http://localhost:5555${path}`, {
        credentials: credentials,
        headers: cookie ? {
            'Cookie': cookie
        } : undefined
    })

    if(request.ok) {
        return await request.json()
    } else {
        console.log(request.url, request.status)
        console.log(request)
        throw new Error(`Request error`)
    }
}