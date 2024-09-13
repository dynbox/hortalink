import { useContext } from "react";
import { SearchContext } from "../SearchContainer";

export default function FilterModal(props: { star_image_src: string }) {
    const { filter_modal_open, set_filter_modal_open } = useContext(SearchContext)

    const star = <img src={props.star_image_src} width={20} height={20} alt="Imagem de uma estrela" />

    if(filter_modal_open) {
        return (
            <div className="filter_modal">
                <div className="filter_modal_container">
                    <h2>Filtros</h2>
                    <div className="line" />
                    <h3>Estrelas</h3>
                    <div className="stairs">
                        <div className="star">
                            {star}
                            <p>(5)</p>
                        </div>
                        <div className="star">
                            {star}
                            <p>(4)</p>
                        </div>
                        <div className="star">
                            {star}
                            <p>(3)</p>
                        </div>
                        <div className="star">
                            {star}
                            <p>(2)</p>
                        </div>
                        <div className="star">
                            {star}
                            <p>(1)</p>
                        </div>
                    </div>
                    <h3>Distância</h3>
                    <div className="distance centered">
                        <label htmlFor="distance">Até: </label>
                        <select id="distance">
                            <option>45km</option>
                        </select>
                    </div>
                    <h3>Categoria</h3>
                    <div className="blocky_text centered">
                        <div className="text">
                            <p>Tuberosas</p>
                        </div>
                        <div className="text">
                            <p>Folhas</p>
                        </div>
                        <div className="text">
                            <p>Frutas</p>
                        </div>
                    </div>
                </div>
            </div>
        )
    } else {
        return <></>
    }
}