import { useState } from "react";

export default function Gallery(props: { photos: string[], arrow_image_src: string }) {
    const photos = ["tomate", "alcachofra", "morango"]
    const [imageIndex, setImageIndex] = useState<number>(0)

    const [firstLoad, setLoaded] = useState<boolean>(false)

    return (
        <>
            <section className="gallery" style={{ minHeight: firstLoad ? 0 : `388px`, minWidth: firstLoad ? 0 : `500px` }}>
                <img
                    src={`http://localhost:5767/resources/products/${photos[imageIndex]}.jpg?size=1024`}
                    alt="Foto do produto"
                    onLoad={() => {
                        if(!firstLoad) {
                            setLoaded(true)
                        }
                    }}
                />
                <div className="arrows">
                    <img className="arrow left" src={props.arrow_image_src} onClick={() => setImageIndex(imageIndex >= 1 ? imageIndex - 1 : imageIndex)}/>
                    <img className="arrow right" src={props.arrow_image_src} onClick={(() => setImageIndex(imageIndex < photos.length ? imageIndex + 1: imageIndex))} />    
                </div>
            </section>
            <div className="bg">
                <img src={`http://localhost:5767/resources/products/${photos[imageIndex]}.jpg?size=1024`} />
            </div>
        </>
    )
}