import { createContext, useEffect, useState } from "react";
import type { Filter } from "../../../interfaces/ProductsFilter";
import Products from "../../../stores/Products";
import get_products from "../../../api/get_products";

interface Setters {
    set_filter: React.Dispatch<React.SetStateAction<Filter>>
}

const SearchContext = createContext<{ filter: Filter } & Setters & { store: string }>({
    filter: {
        page: null,
        per_page: null,
    },
    set_filter: null,
    store: null
})

export default function SearchContainer({ children, store }: { children: JSX.Element[], store: string }) {
    const [filter, set_filter] = useState<Filter>({
        page: 1,
        per_page: 10
    })

    useEffect(() => {
        Products.products_filter.listen((v) => {
            get_products("search_result")
        })   
    }, [])

    return (
        <>
            <SearchContext.Provider value={{ filter, set_filter, store }}>
                {children}
            </SearchContext.Provider>
        </>
    )
}

export {
    SearchContext
}