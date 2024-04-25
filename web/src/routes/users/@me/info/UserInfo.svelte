<script lang="ts">
    import {onMount} from 'svelte';

    let originalData: any = {};
    let avatar: string | null = null;
    let name: string | null = null;
    let phone: number | null = null;
    let email: string | null = null;

    onMount(async () => {
        try {
            const response = await fetch('http://localhost:5443/api/v1/users/@me', {
                credentials: 'include'
            });

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            } else {
                originalData = await response.json();

                avatar = originalData.user.avatar || null;
                name = originalData.user.name || null;
                phone = originalData.user.phone || null;
                email = originalData.user.email || null;
            }
        } catch (error) {
            console.error('There was a problem with the fetch operation: ', error);
        }
    });

    async function handleSubmit(event: Event) {
        event.preventDefault();

        const modifiedFields: any = {};

        if (avatar !== originalData.avatar) modifiedFields.avatar = avatar;
        if (name !== originalData.name) modifiedFields.name = name;
        if (phone !== originalData.phone) modifiedFields.phone = phone;
        if (email !== originalData.email) modifiedFields.email = email;

        if (Object.keys(modifiedFields).length == 0) return;

        try {
            await fetch('http://localhost:5443/api/users/@me', {
                method: 'PATCH',
                credentials: 'include',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(modifiedFields)
            });
        } catch (error) {
            console.error('There was a problem with the fetch operation: ', error);
        }
    }
</script>

<div class="component-elements">
    <form on:submit={handleSubmit}>

        <img src={avatar} alt="">
        <label for="avatar">Editar foto</label>
        <input type="file" name="avatar" id="avatar"/>

        <label for="name">Nome:</label>
        <input bind:value={name} type="text" name="name" id="name"/>
        <label for="name">Número de telefone:</label>
        <input bind:value={phone} type="number" name="phone" id="phone"/>
        <label for="email">Email:</label>
        <input bind:value={email} type="email" name="email" id="email"/>

        <button type="submit">
            Salvar
        </button>
    </form>
</div>

<style>
    .component-elements {
        width: 100%;
        display: grid;
        place-items: center;
    }

    form {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        width: 100%;
        max-width: 425px;
        padding: 1rem;
        border-radius: 0.5rem;
        border: 1px solid hsl(120, 17%, 70%);
        box-shadow: rgba(79, 161, 79, 0.2) 0px 2px 8px 0px;
    }

    img {
        width: 30%;
        border-radius: 30%;
        background-color: #ddd;
    }

    input[type="file"] {
        display: none;
    }

    form > button {
        margin-top: 0.25rem;
        padding: 0.5rem;
        background-color: #50a150;
        color: #fff;
        font-weight: bold;
        letter-spacing: 2px;
    }

    input, button {
        padding: 0.25rem;
        border: 0 solid;
        border-radius: 0.5rem;
        transition: 150ms;
        cursor: pointer;
        outline: none;
    }

    input:hover {
        padding: 0.5rem;
        font-size: large;
    }

    input:focus {
        padding: 0.5rem;
        font-size: large;
        border-bottom: 1px solid hsl(120, 17%, 70%);
    }

    form > input, form > button {
        width: 100%;
    }
</style>
