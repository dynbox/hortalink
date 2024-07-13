import { useState, useEffect } from "react";
import { useStore } from "@nanostores/react"
import Items from "./Items";
import Products from "../../../../stores/Products";
import Geolocation from "../../../../stores/Geolocation";
import geoLocation from "../../../../util/geoLocation";

import call_all from "../../../../api/call_all";


export default function ProductsSection(props: { star_image_src: string, location_image_src: string, arrow_image_src: string}) {
    const { star_image_src, location_image_src, arrow_image_src } = props

    const search_result = useStore(Products.search_result)
    const [requested, setRequested] = useState(false)

    useEffect(() => {
        call_all(setRequested)

        Geolocation.position.listen(() => {
            if(Products.search_result.value?.length) {
                
            } else {
                call_all()
            }
        })

        geoLocation.watchPosition() // chamar apenas no elemento root
    }, [])

    return (
        <>
           {
                !search_result?.length &&
                <>
                    <section className="products_section">
                        <h2>Recentes</h2>
                        {
                            requested &&
                            <Items
                                star_image_src={star_image_src}
                                location_image_src={location_image_src}
                                arrow_image_src={arrow_image_src}
                                container_id="recentes"
                                store="recent"
                            />
                        }
                    </section>
                    <section className="products_section">
                        <h2>Mais pedidos</h2>
                        {
                            requested &&
                            <Items
                                star_image_src={star_image_src}
                                location_image_src={location_image_src}
                                arrow_image_src={arrow_image_src}
                                container_id="mais_pedidos"
                                store="most_requested"
                            />
                        }
                    </section>
                </>
           }
           {
                search_result &&
                <>
                    <section className="products_section">
                        <Items
                            star_image_src={star_image_src}
                            location_image_src={location_image_src}
                            arrow_image_src={arrow_image_src}
                            container_id="searched"
                            store="search_result"
                            noscroll={true}
                        />
                    </section>
                </>
            }
        </>
    )
}