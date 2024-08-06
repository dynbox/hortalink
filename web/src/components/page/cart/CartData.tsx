import { useState, useEffect } from "react";
import Geolocation from "../../../stores/Geolocation";
import { useStore } from "@nanostores/react";

import type { Cart } from "../../../interfaces/Cart";
import degreesToKm from "../../../util/degreesToKm";

function DateInput() {
    return (
        <input
            type="datetime-local"
            alt="Selecionar data de retirada"
        />
    )
}

export default function CartData(props: { cartData: Cart }) {
    const cart = props.cartData
    const pos = useStore(Geolocation.position)
    const pos_state = useStore(Geolocation.state)

    return (
        <>
            <div className="products">
                {
                    cart.product.map((item, i) => (
                        <div className="product_data" key={i}>
                            <input type="checkbox" className="checkbox" />
                            <div className="product">
                                <div>
                                    <img
                                        src={`http://localhost:5767/resources/products/${item.photos ? item.photos[0] : "undefined"}.jpg?size=256`}
                                        width={108}
                                        height={108}
                                        alt={`Foto do produto "${item.name}"`}
                                    />
                                </div>
                                <div>
                                    <h2>{item.name}</h2>
                                    <div className="product_infos">
                                        <p>Distância: {item.dist && pos ? `${degreesToKm(item.dist, pos[0])} km` : pos_state}</p>
                                        <p>Valor: R$ {item.price.toFixed(2)}/{item.unit}</p>
                                    </div>
                                    <p className="price_label">Valor total</p>
                                    <p className="price">R$ {item.price.toFixed(2)}</p>
                                </div>
                            </div>
                            <div className="order_specs">
                                <DateInput />
                                <div className="amount">
                                    <button className="button">
                                        -
                                    </button>
                                    <span>1</span>
                                    <button className="button">
                                        +
                                    </button>
                                </div>
                            </div>
                        </div>
                    ))
                }
            </div>
        </>
    )
}