<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import { fade } from 'svelte/transition';


    const dispatch = createEventDispatcher();

    function goBack(component: string) {
        dispatch('goback', {
            component: component
        })
    }
</script>


<main transition:fade={{ delay: 250, duration: 300 }}>
    <div class="top-bar">
        <button on:click={()=>{goBack('logscreen')}}>voltar</button>
    </div>
    
    <form action="">
        <input type="email" placeholder="e-mail:" />
        <input type="number" placeholder="numero de telefone:" />
        <input type="password" placeholder="senha:" />
        <label for="account-type">tipo de conta:</label>
        <div class="account-type">

            <label for="cliente">
                <input type="radio" name="account-type" id="cliente" />
                cliente
            </label>

            <label for="vendedor">
                <input type="radio" name="account-type" id="vendedor" />
                vendedor
            </label>
        </div>
        <button type="submit">inscrever-se</button>
    </form>
</main>


<style>
    .account-type{
        width: 100%;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }
    .account-type label{
        width: 50%;
        display: flex;
        justify-content: center;
        align-items: center;
        transition: 150ms;
    }
</style>


