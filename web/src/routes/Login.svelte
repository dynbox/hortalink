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
        <input type="email" placeholder="e-mail ou numero de telefone:" />
        <input type="password" placeholder="senha:" />
        <button type="submit">entrar</button>
    </form>
    <p>esqueceu a senha? <a href=" ">clique aqui!</a></p>
</main>


