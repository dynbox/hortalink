<script lang="ts">
    import "$lib/css/seller/sellerprofile.css";
    import {onMount} from "svelte";
    import {page} from "$app/stores";

    let seller: any = {};
    let products: any = [];
    let avatar_url: string | null = null

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

                if (seller.avatar != null) {
                    avatar_url = `http://localhost:5767/avatars/${seller.id}/${seller.avatar}.png?size=128`
                }

                products = data.products;
            }
        } catch (error) {
            console.error(
                "There was a problem with the fetch operation: ",
                error,
            );
        }
    });
</script>
<main>
    <div class="profile-section">
        <div class="avatar-container">
            <img src={avatar_url} alt="Avatar">
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
        {#each products as {id, product, photos, price, rating, quantity}}
            <div class="product-square-container">
                <div class="product-square">
                    <img src={`http://localhost:5767/products/${id}/${photos[0]}.png?size=128`} alt="Foto">
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
