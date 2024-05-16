<script lang="ts">
    import type {EventHandler} from "svelte/elements";
    import ExternalOptions from "./ExternalOptions.svelte";
    import {fade} from 'svelte/transition';
    import defaultProfile from "$lib/assets/default-pricture.svg"

    let password: string | null = null;
    let confirmPassword: string | null = null;

    let email: string | null = null;
    let accountType: number | null = null;
    let name: string | null = null;

    const handleSignup: EventHandler<Event, HTMLFormElement> = async (event) => {
        event.preventDefault();

        const response = await fetch('http://localhost:5443/api/v1/auth/sign-in', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            credentials: 'include',
            body: JSON.stringify({
                email,
                role: Number(accountType),
                password,
                name
            })
        });

        if (response.ok) {
            window.location.href = '/';
        }
    }

    let componentRendered = "first"

    function changeComponent(component: string) {
        componentRendered = ""
        setTimeout(() => {
            componentRendered = component
        }, 300)
    }
</script>

{#if componentRendered === "first"}
    <form action="" transition:fade={{duration: 300 }} on:submit|preventDefault={() => changeComponent("second")}>
        <input bind:value={email} name="email" type="email" placeholder="E-mail:" required />
        <input bind:value={password} name="password" type="password" placeholder="Senha:" required />
        <input bind:value={confirmPassword} type="password" name="confirmPassword" placeholder="Confirme a senha:" required />

        <button>Próximo</button>

        <ExternalOptions/>
    </form>
{:else}
    {#if componentRendered === "second"}
        <form action="" on:submit|preventDefault={handleSignup}>
            <div class="profile-container">
                <img src={defaultProfile} alt="Avatar">
            </div>

            <input bind:value={name} name="name" type="text" placeholder="Nome:"/>

            <label for="account-type" style="padding-top: 10px">Eu sou:</label>
            <div class="account-type">
                <label for="client">
                    <input bind:group={accountType} type="radio" name="account-type" id="client" value=3/>
                    Cliente
                </label>

                <label for="seller">
                    <input bind:group={accountType} type="radio" name="account-type" id="seller" value=4/>
                    Vendedor
                </label>
            </div>

            <button>Cadastrar</button>
        </form>
    {/if}
{/if}

<style>
    .account-type {
        display: grid;
        grid-template-columns: repeat(2, 1fr);
        gap: 10px;
        width: 100%;
    }

    .account-type label {
        display: flex;
        justify-content: center;
        align-items: center;
        padding: 10px;
        border: 1px solid #ccc;
        border-radius: 5px;
        transition: background-color 150ms;
    }

    .account-type label:hover {
        background-color: #f0f0f0;
    }

    .account-type input[type="radio"] {
        margin-right: 10px;
    }

    .profile-container {
        width: 50%;
        display: grid;
        place-items: center;
        aspect-ratio: 1/1;
    }

    .profile-container img {
        width: 80%;
        aspect-ratio: 1/1;
        border-radius: 100%;
    }
</style>