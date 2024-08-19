import Products from "../../../../stores/Products";
import { useStore } from "@nanostores/react";
import Geolocation from "../../../../stores/Geolocation";
import degreesToKm from "../../../../util/degreesToKm";

export default function Items(props: { container_id: string, store: string, star_image_src: string, location_image_src: string, arrow_image_src: string, noscroll?: boolean }) {
    const products = useStore(Products.search_result)
    const pos = useStore(Geolocation.position)

    return (
        <>  
            <section className="products_column">
                <div className="products_column_content">
                    {
                        products.length !== 0 && products.map((item, i) => (
                            <div className="product" key={i}>
                                <div>
                                    <img
                                        src={`cdn/products/${item.id}/${encodeURIComponent(item.photos ? item.photos[0] : "undefined")}.jpg?size=256`}
                                        width={108}
                                        height={108}
                                        alt={`Foto do produto "${item.product.name}"`}
                                    />
                                </div>
                                <div>
                                    <h2>{item.product.name}</h2>
                                    <div className="product_infos">
                                        <p>Distância: {item.dist ? degreesToKm(item.dist, pos[0]) : "N/A"} km</p>
                                        <p>Valor por {item.unit}: {item.price}</p>
                                    </div>
                                    <div className="product_modal_rating">
                                        <img
                                            src={props.star_image_src}
                                            alt="Imagem de uma estrela, ao lado direito está indicando o número de avaliações."
                                            width={14}
                                            height={14}
                                        />
                                        <p>({item.rating_quantity || "N/A "})</p>
                                    </div>
                                    <p className="price">R$ {item.price}</p>
                                </div>
                            </div>
                        ))
                    }
                </div>
            </section>
        </>
    )
}