import { useEffect, useState } from "react";
import { useStore } from "@nanostores/react";
import Products from "../../../../stores/Products";
import ItemsModal from "./ItemsModal";
import Geolocation, { GPS_state } from "../../../../stores/Geolocation";
import degreesToKm from "../../../../util/degreesToKm";

import { useMemo } from "react";

function ItemDist(props: { id: number }) {
    const dists = useStore(Products.all_products_dist)
    console.log(dists)
    return <>{dists ? degreesToKm(dists[props.id]) : "N/A"} km</>
}

export default function Items(props: { container_id: string, store: string, star_image_src: string, location_image_src: string, arrow_image_src: string, noscroll?: boolean }) {
    const items = useStore(Products[props.store]);
    const local_permission_state = useStore(Geolocation.state)
    //const pos = useStore(Geolocation.position)

    console.log(items)

    if(props.store === "search_result") {
        return (
            <ItemsModal {...props} />
        )
    } else {
        const [slide_pos, set_slide_pos] = useState<number>(0)

        /*useEffect(() => {
            Geolocation.position.listen(new_position => {
                const latitude = new_position[0]
                const longitude = new_position[1]
    
                // função para obter apenas a distância pela API (ainda não tem)
            })
        }, [])*/
    
        useEffect(() => {
            const items_element = document.querySelectorAll<HTMLElement>(`#${props.container_id} .product_list .product_container .product`)
            const prev_element = document.querySelector<HTMLElement>(`#${props.container_id} .arr-prev`)
            const next_element = document.querySelector<HTMLElement>(`#${props.container_id} .arr-next`)
    
            if(slide_pos === 0) {
                prev_element.style.display = "none"
            } else {
                prev_element.style.display = "block"
            }
    
            if(slide_pos >= items.length - 1) {
                next_element.style.display = "none"
            } else {
                next_element.style.display = "block"
            }

        }, [slide_pos])
    
        return (
            <>
                <section className={`products ${props.noscroll ? "products_noscroll" : ""}`} id={props.container_id}>
                    <div className="arrow_container arr-prev" onClick={(() => slide_pos >= 1 ? set_slide_pos(slide_pos - 1) : console.log("?"))}>
                        <img src={props.arrow_image_src}
                            alt="Seta para direita, clique para passar os elementos do carrossel."
                            width={35}
                            height={35}
                            style={{ transform: "rotate(180deg)" }}
                        />
                    </div>
                    <div className={`product_list`}>
                        <div className="product_container">
                            {
                                items && items.map((item, i) => {
                                    return (
                                        <a href={`/users/${item.seller_id}/products/${item.id}`} className={`product ${i >= slide_pos && !props.noscroll ? "" : `${props.noscroll ? "" : "hidden"}`}`} key={`${props.container_id}-${Math.floor(Math.random() * 100) + Date.now()}`}>
                                            <div className="head">
                                                <h3>{item.product.name}</h3>
                                                {
                                                    item.rating_quantity &&
                                                    <span>
                                                        <img
                                                            src={props.star_image_src}
                                                            alt="Imagem de uma estrela, ao lado direito está indicando o número de avaliações."
                                                            width={12}
                                                            height={12}
                                                        />
                                                        <p>({item.rating_quantity})</p>
                                                    </span>
                                                }
                                            </div>
                                            <img 
                                                src={`${import.meta.env.PUBLIC_FRONTEND_CDN_URL}/products/${item.id}/${encodeURIComponent(item.photos[0])}.jpg?size=256`}
                                                width={145}
                                                height={138}
                                                alt={`Foto do produto "${item.product.name}"`}
                                            />
                                            <div className="footer">
                                                <span>
                                                    <img
                                                        src={props.location_image_src}
                                                        alt="Ícone de GPS, ao lado está a sua distância até o vendedor, em quilômetros."
                                                        width={15}
                                                        height={15}
                                                    />
                                                    <p><ItemDist id={item.id} /></p>
                                                </span>
                                                <span className="price">
                                                    <p>R$ {item.price} {item.unit}</p>
                                                </span>
                                            </div>
                                        </a>
                                    )
                                })
                            }
                        </div>
                    </div>
                    <div className="arrow_container arr-next" onClick={(() => set_slide_pos(slide_pos + 1))}>
                        <img src={props.arrow_image_src}
                            alt="Seta para direita, clique para passar os elementos do carrossel."
                            width={35}
                            height={35}
                        />
                    </div>
                </section>
            </>
        )
    }
}