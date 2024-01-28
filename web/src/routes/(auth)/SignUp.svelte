<script lang="ts">
    import type { EventHandler } from "svelte/elements";

    let email: string = '';
    let phone: string = '';
    let password: string = '';
    let accountType: string = '';

    const handleSignup: EventHandler<Event, HTMLFormElement> = async (event) => {
        event.preventDefault();

        const response = await fetch('http://localhost:5443/api/auth/sign', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            credentials: 'include',
            body: JSON.stringify({
                email,
                phone,
                password,
                name: accountType
            })
        });

        if (response.ok) {
            window.location.href = '/';
        }
    }
</script>

<form on:submit|preventDefault={handleSignup}>
    <input bind:value={email} name="email" type="email" placeholder="E-mail:" />
    <input bind:value={phone} name="phone" type="number" placeholder="Número de telefone:" />
    <input bind:value={password} name="password" type="password" placeholder="Senha:" />
    <label for="account-type">Tipo de conta:</label>

    <div class="account-type">
        <label for="cliente">
            <input bind:group={accountType} type="radio" name="account-type" id="cliente" value="cliente" />
            Cliente
        </label>

        <label for="vendedor">
            <input bind:group={accountType} type="radio" name="account-type" id="vendedor" value="vendedor" />
            Vendedor
        </label>
    </div>
    <button type="submit">Cadastrar</button>
</form>