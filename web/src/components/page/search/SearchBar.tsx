import { useContext } from "react";
import { SearchContext } from "./SearchContainer";
import Products from "../../../stores/Products";

import Images from "../../../stores/Images";

export default function SearchBar() {
    const images = Images.get()
    console.log(images)
    return (
        <>
            <section className="search_bar">
                <input type="text" placeholder="Procurar por..." />
                <div className="icon_box">
                    <img
                        src={images["search.svg"]}
                        alt="Ícone de lupa, ao ser clicado, pesquisa o que você escreveu no input de texto."
                        width={22}
                        height={22}
                        style={{ transform: "translateX(-10px)" }}
                    />
                </div>
            </section>
        </>
    )
}