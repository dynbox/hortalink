<script lang="ts">
    import '$lib/css/auth/acess.css';
    import google from '$lib/assets/google.svg';
    import facebook from '$lib/assets/facebook.svg';
    import linkedin from '$lib/assets/linkedin.svg';
    import { fade } from 'svelte/transition';
    import Login from "../Login.svelte";
    import Signup from "../SignUp.svelte";

    let componentRendered = "access"

    function changeComponent(component: string){
        componentRendered = ""
        setTimeout(() => {
            componentRendered = component
        }, 300)
    }

    async function handleExternalLogin(oauthType: string) {
        try {
            const response = await fetch(`http://localhost:5443/api/oauth/${oauthType}`, {
                method: 'POST',
                credentials: 'include'
            });

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            } else {
                let data = await response.json()

                window.location.href = data.auth_url, '_blank';
            }
        } catch (error) {
            console.error('There was a problem with the fetch operation: ', error);
        }
    }
</script>

<main class="fader">
    {#if componentRendered === "access"}
        <form action="" transition:fade={{duration: 300 }}>
            <button on:click={()=>{changeComponent("login")}}>Entrar</button>
            <button on:click={()=>{changeComponent("signup")}}>Cadastre-se</button>
            <div class="external-options">
                <button type="button" on:click={() => handleExternalLogin('google')}>
                    <img src={google} alt="google logo">
                    <span>Iniciar com Google</span>
                </button>
                <button type="button" on:click={() => handleExternalLogin('facebook')}>
                    <img src={facebook} alt="facebook logo">
                    <span>Iniciar com Facebook</span>
                </button>
                <button type="button" on:click={() => handleExternalLogin('linkedin')}>
                    <img src={linkedin} alt="linkedin logo">
                    <span>Iniciar com Linkedin</span>
                </button>
            </div>
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
        width: 35px;
    }
</style>
