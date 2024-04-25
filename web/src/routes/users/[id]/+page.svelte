<script lang="ts">
    import '$lib/css/seller/sellerprofile.css';
    import {onMount} from "svelte";
    import {page} from '$app/stores';

    let seller: any = {}
    let products: any = []

    onMount(async () => {
        try {
            const response = await fetch('http://localhost:5443/api/v1/sellers/' + $page.params.id, {
                credentials: 'include'
            });

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            } else {
                let data = await response.json();

                seller = data.seller;
                products = data.products;
            }
        } catch (error) {
            console.error('There was a problem with the fetch operation: ', error);
        }
    })
</script>

<main>
    <div class="profile-section">
        <div class="avatar-container">
            <div></div> <!--tag who will contain the avatar in their background-image css-->
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
            <div class="product-square">
                <div class="img-container">
                </div>
                <div class="product-data">
                    <span class="product-name">
                        {product.name}
                    </span>
                    <div>
                        <span class="price">{price}</span>
                        <span class="rating">{rating}</span>
                        <span class="quantity">{quantity}</span>
                    </div>
                </div>
            </div>
        {/each}
    </div>
</main>