import { useEffect } from "react";
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
]

export default function Search(props: { search_icon_url: string }) {
    useEffect(() => {
        const icon_box = document.querySelector<HTMLDivElement>(".search_bar .icon_box")

        icon_box.addEventListener("click", async () => {
            Products.searched.set(sample_data)
        })
    }, [])

    return (
        <>
            <section className="search_bar">
                <input type="text" placeholder="Procurar por..." />
                <div className="icon_box">
                    <img
                        src={props.search_icon_url}
                        alt="Ícone de lupa, ao ser clicado, pesquisa o que você escreveu no input de texto."
                        width={32}
                        height={32}
                        style={{ transform: "scale(1.25) translateX(-5px)" }}
                    />
                </div>
            </section>
        </>
    )
}