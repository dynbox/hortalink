<script lang="ts">
    import '$lib/css/auth/acess.css';
    import { fade } from 'svelte/transition';
    import Login from "../Login.svelte";
    import Signup from "../SignUp.svelte";
    import ExternalOptions from "../ExternalOptions.svelte";

    let componentRendered = "access"

    function changeComponent(component: string){
        componentRendered = ""
        setTimeout(() => {
            componentRendered = component
        }, 300)
    }
</script>

<main class="fader">
    {#if componentRendered === "access"}
        <form action="" transition:fade={{duration: 300 }}>
            <button on:click={()=>{changeComponent("login")}}>Entrar</button>
            <button on:click={()=>{changeComponent("signup")}}>Cadastre-se</button>

            <ExternalOptions/>
        </form>
    {:else}
        <div class="fader" transition:fade={{ delay: 250, duration: 300 }}>
            <div class="top-bar">
                <button on:click={()=> changeComponent('access')}>Voltar</button>
            </div>

            {#if componentRendered === "login"}
                <Login/>
            {:else if componentRendered === "signup"}
                <Signup/>
            {/if}
        </div>
    {/if}
</main>
