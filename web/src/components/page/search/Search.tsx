import { useStore } from "@nanostores/react"
import { PageType } from "../../../stores/SearchPagination"

import SearchPagination from "../../../stores/SearchPagination"
import SearchMenu from "./SearchMenu"

export default function SearchMain(props: { store: string }) {
    const type = useStore(SearchPagination.pageType)

    switch(type) {
        case PageType.SearchHome:
            return (
                <SearchMenu store={props.store} />
            )
    }
}