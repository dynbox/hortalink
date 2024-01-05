<script lang="ts">
    import Login from "./Login.svelte";
    import Signup from "./Signup.svelte";
    import { fade } from 'svelte/transition';



    let componentRendered = "logscreen"

    function changeComponent(component: string){
        componentRendered = ""
        setTimeout(()=>{componentRendered = component}, 300)
        
    }
</script>


<main>
        {#if componentRendered=="logscreen"}
            <form action="" transition:fade={{duration: 300 }}>
                <button on:click={()=>{changeComponent("login")}}>entrar</button>
                <button on:click={()=>{changeComponent("signup")}}>cadastre-se</button>
                <div class="external-options">
                    <button type="button">google</button>
                    <button type="button">facebook</button> 
                    <button type="button">apple</button>
                </div>
            </form>
            
        {:else if componentRendered=="login"}
            <Login on:goback={(event)=>{changeComponent(event.detail.component)}}/>

        {:else if componentRendered=="signup"}
            <Signup on:goback={(event)=>{changeComponent(event.detail.component)}}/>

        {/if}
</main>


<style>
    @import url('./Logscreen.css');

    .external-options{
        width: 100%;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }
    .external-options > button{
        width: 32%;
    }
</style>
