import { useState, useContext } from "react";
import { SearchContext } from "./SearchContainer";
import Products from "../../../stores/Products";
import SearchContainer from "./SearchContainer";
import SearchBar from "./SearchBar";
import Tags from "./Tags";

import UnpaginatedProductList from "../../../layouts/UnpaginatedProductList";
import Images from "../../../stores/Images";

export default function SearchMenu(props: { store: string }) {
    const images = Images.get()

    return (
        <>
            <SearchContainer store={props.store}>
                <SearchBar />
                <div className="line" />
                <Tags />
                <UnpaginatedProductList
                    arrow_image_src={images["arrow.svg"]}
                    location_image_src={images["location.svg"]}
                    star_image_src={images["star.svg"]}
                    store={props.store}
                />
            </SearchContainer>
        </>
    )
}