import SearchMenu from "./SearchMenu";
import SearchResults from "./results/SearchResults";

import SearchContainer from "./SearchContainer";
import SearchBar from "./SearchBar";
import Tags from "./Tags";
import { useContext } from "react";
import { ResultType, SearchResultContext } from "./results/SearchResultContainer";

import { useStore } from "@nanostores/react";
import Products from "../../../stores/Products";

function SearchScreens(props: { store: string }) {
    const searched = useStore(Products.search_result)
    console.log(searched)

    return (
        <>
            { searched.length === 0 && <SearchMenu {...props} />}
            { searched.length > 0 && <SearchResults /> }
        </>
    )
}

export default function SearchComponent(props: { store: string }) {
    const { result_type } = useContext(SearchResultContext)

    return (
        <>
            <SearchContainer store={props.store}>
                <SearchBar />
                <div className="line" />
                <Tags />

                <SearchScreens {...props} />

            </SearchContainer>
        </>
    )
}