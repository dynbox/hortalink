import { useEffect, useState } from "react";
import { useStore } from "@nanostores/react";
import Products from "../../../../stores/Products";

const sample_data = [
    {
        id: 1,
        product: {
            name: "Cenoura",
            id: 1
        },
        price: 29.99,
        rating: 5,
        rating_quantity: 34,
        photos: ["cenoura"],
        dist: 0.7,
        unit: "kg"
    },
    {
        id: 2,
        product: {
            name: "Tomate",
            id: 2
        },
        price: 29.99,
        rating: 5,
        rating_quantity: 34,
        photos: ["tomate"],
        dist: 0.7,
        unit: "kg"
    },
    {
        id: 3,
        product: {
            name: "Beterraba",
            id: 3
        },
        price: 29.99,
        rating: 5,
        rating_quantity: 34,
        photos: ["beterraba"],
        dist: 0.7,
        unit: "kg"
    },
    {
        id: 4,
        product: {
            name: "Gengibre",
            id: 4
        },
        price: 29.99,
        rating: 5,
        rating_quantity: 34,
        photos: ["gengibre"],
        dist: 0.7,
        unit: "kg"
    },
    {
        id: 5,
        product: {
            name: "Gengibre",
            id: 5
        },
        price: 29.99,
        rating: 5,
        rating_quantity: 34,
        photos: ["gengibre"],
        dist: 0.7,
        unit: "kg"
    },
    {
        id: 6,
        product: {
            name: "Gengibre",
            id: 6
        },
        price: 29.99,
        rating: 5,
        rating_quantity: 34,
        photos: ["gengibre"],
        dist: 0.7,
        unit: "kg"
    },
    {
        id: 7,
        product: {
            name: "Gengibre",
            id: 7
        },
        price: 29.99,
        rating: 5,
        rating_quantity: 34,
        photos: ["gengibre"],
        dist: 0.7,
        unit: "kg"
    },
    {
        id: 8,
        product: {
            name: "Gengibre",
            id: 8
        },
        price: 29.99,
        rating: 5,
        rating_quantity: 34,
        photos: ["gengibre"],
        dist: 0.7,
        unit: "kg"
    },
    {
        id: 9,
        product: {
            name: "Gengibre",
            id: 9
        },
        price: 29.99,
        rating: 5,
        rating_quantity: 34,
        photos: ["gengibre"],
        dist: 0.7,
        unit: "kg"
    },
    {
        id: 10,
        product: {
            name: "Gengibre",
            id: 10
        },
        price: 29.99,
        rating: 5,
        rating_quantity: 34,
        photos: ["gengibre"],
        dist: 0.7,
        unit: "kg"
    }
]

Products.recent.set(sample_data)
export default function Items(props: { container_id: string, store: string, star_image_src: string, location_image_src: string, arrow_image_src: string, noscroll?: boolean }) {
    const items = useStore(Products[props.store]);

    const [slide_pos, set_slide_pos] = useState<number>(0)

    useEffect(() => {
        const items_element = document.querySelectorAll<HTMLElement>(`#${props.container_id} .product_list .product_container .product`)
        const prev_element = document.querySelector<HTMLElement>(`#${props.container_id} .arr-prev`)
        const next_element = document.querySelector<HTMLElement>(`#${props.container_id} .arr-next`)

        if(slide_pos === 0) {
            prev_element.style.display = "none"
        } else {
            prev_element.style.display = "block"
        }

        if(slide_pos + 1 > items.length) {
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
                            items && items.map((item, i) => (
                                <div className={`product ${i >= slide_pos && !props.noscroll ? "" : `${props.noscroll ? "" : "hidden"}`}`} key={item.id}>
                                    <div className="head">
                                        <h3>{item.product.name}</h3>
                                        <span>
                                            <img
                                                src={props.star_image_src}
                                                alt="Imagem de uma estrela, ao lado direito está indicando o número de avaliações."
                                                width={12}
                                                height={12}
                                            />
                                            <p>({item.rating_quantity})</p>
                                        </span>
                                    </div>
                                    <img 
                                        src={`http://localhost:5767/resources/products/${item.photos[0]}.jpg?size=256`}
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
                                            <p>{item.dist}Km</p>
                                        </span>
                                        <span className="price">
                                            <p>R$ {item.price} {item.unit}</p>
                                        </span>
                                    </div>
                                </div>
                            ))
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