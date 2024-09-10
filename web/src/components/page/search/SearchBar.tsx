import { useContext, useEffect, useRef, useState } from "react";
import { SearchContext } from "./SearchContainer";
import Products from "../../../stores/Products";
import SearchTypes from "../../../api/searchTypes";

import Images from "../../../stores/Images";
import { useStore } from "@nanostores/react";

async function doSeach(query: string) {
    if(!query || !query.length) {
        return Products.searched.set([])
    }

    const data = await SearchTypes(query, 1, 10)
    Products.searched.set(data)
}

function SearchResults() {
    const results = useStore(Products.searched)

    return (
        <section className="select_result_container">
            {
                results && results.map((result, i) => (
                    <div className="result" key={`option-${result.product_name}`} tabIndex={0}>
                        {result.product_name}
                    </div>
                ))
            }
        </section>
    )
}

export default function SearchBar() {
    const [hidden, setHidden] = useState(true)

    const images = Images.get()
    const inputRef = useRef<HTMLInputElement>()

    function isFocusedOption() {
        console.log(document.activeElement.className)
        if(document.activeElement.className === "result") {
            return true
        } else {
            return false
        }
    }

    useEffect(() => {
        let timeout;

        inputRef.current.addEventListener("input", () => {
            clearTimeout(timeout)

            timeout = setTimeout(() => {
                doSeach(inputRef.current.value)
            }, 800)
        })

        inputRef.current.addEventListener("focus", () => {
            setHidden(false)
        })

        inputRef.current.addEventListener("blur", () => {
            if(isFocusedOption()) {
                setHidden(false)
            } else {
                setHidden(true)
            }
        })
    }, [])

    return (
        <>
            <section className="search_bar">
                <input type="text" placeholder="Procurar por..." ref={inputRef} />
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
            {
                !hidden && <SearchResults />
            }
        </>
    )
}