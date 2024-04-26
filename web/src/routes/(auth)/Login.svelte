<script lang="ts">
    import type { EventHandler } from "svelte/elements";
    import ExternalOptions from "./ExternalOptions.svelte";

    let email: string = '';
    let password: string = '';

    const handleSubmit: EventHandler<Event, HTMLFormElement> = async (event) => {
        event.preventDefault();

        const response = await fetch('http://localhost:5443/api/v1/auth/login', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            credentials: 'include',
            body: JSON.stringify({
                email,
                password
            })
        });

        if (response.ok) {
            window.location.href = "/"
        }
    }
</script>

<form on:submit|preventDefault={handleSubmit}>
    <input bind:value={email} name="email" type="email" placeholder="Email ou número de telefone:" required />
    <input bind:value={password} name="password" type="password" placeholder="Senha:" required />
    <button type="submit">Entrar</button>
    <ExternalOptions/>
</form>
<p>Esqueceu sua senha? <a href=" ">Clique aqui!</a></p>