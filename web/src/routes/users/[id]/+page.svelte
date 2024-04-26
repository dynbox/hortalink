<script lang="ts">
    import "$lib/css/seller/sellerprofile.css";
    import { onMount } from "svelte";
    import { page } from "$app/stores";

    let seller: any = {};
    let products: any = [];

    onMount(async () => {
        try {
            const response = await fetch(
                "http://localhost:5443/api/v1/sellers/" + $page.params.id,
                {
                    credentials: "include",
                },
            );

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            } else {
                let data = await response.json();

                seller = data.seller;
                products = data.products;
            }
        } catch (error) {
            console.error(
                "There was a problem with the fetch operation: ",
                error,
            );
        }
    });

    function fakeonamount() {
        //não consegui arrumar minha db local (┬┬﹏┬┬)
        seller = {
            id: 8,
            name: "Sherlock Holmes",
            avatar: null,
            bio: null,
        };
        products = [
            {
                id: 3,
                product: { id: 3, name: "Product 3" },
                photos: ["photo5.jpg", "photo6.jpg"],
                quantity: 20,
                price: "100.00",
                rating: 4.0,
            },
            {
                id: 8,
                product: { id: 8, name: "Product 8" },
                photos: ["photo15.jpg", "photo16.jpg"],
                quantity: 40,
                price: "150.00",
                rating: 3.0,
            },
            {
                id: 10,
                product: { id: 10, name: "Product 10" },
                photos: ["photo19.jpg", "photo20.jpg"],
                quantity: 50,
                price: "200.00",
                rating: null,
            },
        ];
    }
    fakeonamount();
</script>
<main>
    <div class="profile-section">
        <div class="avatar-container">
            <div></div>
            <!--tag who will contain the avatar in their background-image css-->
        </div>
        <div class="user-info-container">
            <div>
                <span class="user-name">{seller.name}</span>
                <span class="user-rating"></span>
            </div>
            <span class="user-bio"><i>bio:</i> {seller.bio}</span>
        </div>
    </div>
    <div class="products-section">
        {#each products as { id, product, photos, price, rating, quantity }}
            <div class="product-square-container">
                <div class="product-square">
                    <div class="img-container"></div>
                    <div class="product-data">
                        <span class="product-name">
                            {product.name}
                        </span>
                        <span class="product-price">{price} R$</span>
                        <div>
                        <span class="product-rating">{#each {length: rating} as _}
                            <span>*</span>
                            {/each}
                        </span>
                        <span class="product-quantity">{quantity}</span>
                    </div>
                    </div>
                </div>
            </div>
        {/each}
    </div>
</main>
