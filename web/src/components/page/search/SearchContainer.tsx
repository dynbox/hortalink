import { createContext, useState } from "react";
import type { Filter } from "../../../interfaces/ProductsFilter";

interface Setters {
    set_filter: React.Dispatch<React.SetStateAction<Filter>>
}

const SearchContext = createContext<Filter & Setters>({
    page: null,
    per_page: null,
    set_filter: null
})

export default function SearchContainer({ children }: { children: JSX.Element[] }) {
    const [filter, set_filter] = useState<Filter>({
        page: 1,
        per_page: 10
    })

    return (
        <>
            <SearchContext.Provider value={{ ...filter, set_filter }}>
                {children}
            </SearchContext.Provider>
        </>
    )
}

export {
    SearchContext
}