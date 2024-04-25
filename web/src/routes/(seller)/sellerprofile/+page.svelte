<script>
    import '$lib/css/seller/sellerprofile.css';
    const data = {
        user: {
            id: 8, 
            name: "Sherlock Holmes", 
            avatar: null, 
            bio: "o caba é bom."
        },
        products: [
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
        ],
    };
</script>

<main>
    <div class="profile-section">
        <div class="avatar-container">
            <div></div> <!--tag who will contain the avatar in their background-image css-->
        </div>
        <div class="user-info-container">
            <div>
                <span class="user-name">{data.user.name}</span>
                <span class="user-rating"></span>
            </div>
            <span class="user-bio"><i>bio:</i> {data.user.bio}</span>

        </div>
    </div>
    <div class="products-section">
        {#each data.products as {id, product, photos, price, rating, quantity}}
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

<style>

</style>
