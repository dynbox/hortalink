import SearchResultContainer, { SearchResultContext } from "./SearchResultContainer";
import { useContext } from "react";

import Selector from "./Selector";
import { useStore } from "@nanostores/react";

import Products from "../../../../stores/Products";
import Images from "../../../../stores/Images";
import UnpaginatedProductList from "../../../../layouts/UnpaginatedProductList";

function ProductsList() {
    const images = Images.get()
  
    return (
        <>
            {
                <UnpaginatedProductList
                    arrow_image_src={images["arrow.svg"]}
                    location_image_src={images["location.svg"]}
                    star_image_src={images["star.svg"]}
                    store={"search_result"}
                    isColumn={true}
                    noInitialFetch={true}
                />
            }
        </>
    )
}

export default function SearchResults() {
    return (
        <>
            <SearchResultContainer store="search_result">
                <Selector />
                <ProductsList />
            </SearchResultContainer>
        </>
    )
}