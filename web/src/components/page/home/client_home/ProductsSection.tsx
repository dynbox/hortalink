import { useState, useEffect } from "react";
import { useStore } from "@nanostores/react"
import Items from "./Items";
import Products from "../../../../stores/Products";

export default function ProductsSection(props: { star_image_src: string, location_image_src: string, arrow_image_src: string}) {
    const { star_image_src, location_image_src, arrow_image_src } = props

    const searched = useStore(Products.searched)

    return (
        <>
           {
                !searched.length &&
                <>
                    <section className="products_section">
                        <h2>Recentes</h2>
                        <Items
                            star_image_src={star_image_src}
                            location_image_src={location_image_src}
                            arrow_image_src={arrow_image_src}
                            container_id="recentes"
                            store="recent"
                        />
                    </section>
                    <section className="products_section">
                        <h2>Mais pedidos</h2>
                        <Items
                            star_image_src={star_image_src}
                            location_image_src={location_image_src}
                            arrow_image_src={arrow_image_src}
                            container_id="mais_pedidos"
                            store="recent"
                        />
                    </section>
                </>
           }
           {
            searched.length &&
            <>
                    <section className="products_section">
                        <Items
                            star_image_src={star_image_src}
                            location_image_src={location_image_src}
                            arrow_image_src={arrow_image_src}
                            container_id="searched"
                            store="searched"
                            noscroll={true}
                        />
                    </section>
            </>
           }
        </>
    )
}