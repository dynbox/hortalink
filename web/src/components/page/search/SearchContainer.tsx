import { createContext, useState } from "react";
import type { Filter } from "../../../interfaces/ProductsFilter";

interface Setters {
    set_filter: React.Dispatch<React.SetStateAction<Filter>>
}

const SearchContext = createContext<Filter & Setters & { store: string }>({
    page: null,
    per_page: null,
    set_filter: null,
    store: null
})

export default function SearchContainer({ children, store }: { children: JSX.Element[], store: string }) {
    const [filter, set_filter] = useState<Filter>({
        page: 1,
        per_page: 10
    })

    return (
        <>
            <SearchContext.Provider value={{ ...filter, set_filter, store }}>
                {children}
            </SearchContext.Provider>
        </>
    )
}

export {
    SearchContext
}