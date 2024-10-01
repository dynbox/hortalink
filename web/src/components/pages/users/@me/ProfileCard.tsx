import { useStore } from "@nanostores/react"
import Session from "@stores/Session"

export default function ProfileCard() {
    const sessionData = useStore(Session)

    console.log(sessionData)

    return (
        <div className="profile_card">
            <div className="img_container">
                <img
                    src={`${import.meta.env.PUBLIC_FRONTEND_CDN_URL}/resources/products/cebola.jpg?size=256`}
                    width={113}
                    height={113}
                    alt="Sua foto de perfil"
                />
            </div>
            <h2>{sessionData && sessionData.profile.name} {!sessionData && "..."}</h2>
            <div className="stats">
                <div>
                    <p className="title">{sessionData && sessionData.orders.length}</p>
                    <p>Pedidos</p>
                </div>
                <div>
                    <p className="title">0</p>
                    <p>Avaliações</p>
                </div>
                <div>
                    <p className="title">0</p>
                    <p>Seguindo</p>
                </div>
            </div>
        </div>
    )
}