import type { Cart } from "../../../interfaces/Cart";
import type { RawCart } from "../../../interfaces/RawCart";
import CartData from "./CartData";

export default function CartGroup(props: { carts: RawCart[] }) {
    const Carts: Cart[] = []

    for(const cart of props.carts) {
        const current_cart_index = Carts.findIndex((matching_cart) => matching_cart.seller.id === cart.seller.id)
        
        if(current_cart_index === -1) {
            
            Carts.push({
                ...cart,
                product: [
                    {
                        ...cart.product,
                        amount: cart.amount
                    }
                ]
            })           
        } else {
            Carts[current_cart_index].product.push({ ...cart.product, amount: cart.amount })
        }
    }

    return (
        <>
            {
                Carts.map((cart, i) => (
                    <section className="cart-container" key={i}>
                        <div className="header">
                            <div className="info">
                                <div className="image">
                                    <img
                                        width={53}
                                        height={53}
                                        alt={`Foto de perfil do vendedor ${cart.seller.name}`}
                                        src={`${import.meta.env.PUBLIC_FRONTEND_CDN_URL}/avatars/${cart.seller.id}/${cart.seller.avatar}.jpg?size=256`}
                                    />
                                </div>
                                <h2>{cart.seller.name}</h2>
                            </div>
                            <div className="see-profile">
                                <a>Ver perfil</a>
                            </div>
                        </div>
                        <div className="content">
                            {
                                Carts.map((cart, i) => (
                                    <CartData 
                                        cartData={cart}
                                        key={i}
                                    />
                                ))
                            }
                        </div>
                    </section>
                ))
            }
        </>
    )
}