<script lang="ts">
    import '$lib/css/auth/acess.css';
    import google from "$lib/google.png";
    import facebook from "$lib/facebook.png";
    import linkedin from "$lib/linkedin.png";
    import { fade } from 'svelte/transition';

    import Login from "../Login.svelte";
    import Signup from "../SignUp.svelte";
    


    import { createEventDispatcher } from 'svelte';

    let componentRendered = "access"

    function changeComponent(component: string){
        componentRendered = ""
        setTimeout(() => {
            componentRendered = component
        }, 300)
    }


    
    const dispatch = createEventDispatcher();


</script>

<main class="fader">
    {#if componentRendered === "access"}
        <form action="" transition:fade={{duration: 300 }}>
            <button on:click={()=>{changeComponent("login")}}>Entrar</button>
            <button on:click={()=>{changeComponent("signup")}}>Cadastre-se</button>
            <div class="external-options">
                <button type="button">
                    <img src={google} alt="google logo">
                    <span>iniciar com Google</span>
                </button>
                <button type="button">
                    <img src={facebook} alt="facebook logo">
                    <span>iniciar com Facebook</span>
                </button>
                <button type="button">
                    <img src={linkedin} alt="linkedin logo">
                    <span>iniciar com Linkedin</span>
                </button>
            </div>
        </form>

    {:else if componentRendered === "login"}
    <div class="fader" transition:fade={{ delay: 250, duration: 300 }}>
        <div class="top-bar">
            <button on:click={()=> changeComponent('access')}>Voltar</button>
        </div>

        <Login/>
    </div>

    {:else if componentRendered === "signup"}
        <div class="fader" transition:fade={{ delay: 250, duration: 300 }}>
            <div class="top-bar">
                <button on:click={()=> changeComponent('access')}>Voltar</button>
            </div>

            <Signup/>
        </div>

    {/if}
</main>

<style>
    .external-options {
        width: 100%;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .external-options > button {
        width: 32%;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
    }
    .external-options > button > img {
        width: 30%;
    }

</style>
