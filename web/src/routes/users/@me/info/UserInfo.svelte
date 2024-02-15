<script lang="ts">
    import { onMount } from 'svelte';

    let originalData: any = {};
    let avatar = "";
    let username = "";
    let name = "";
    let phone = "";
    let address = "";
    let email = "";

    onMount(async () => {
        try {
            const response = await fetch('http://localhost:5443/api/v1/users/me', {
                credentials: 'include'
            });

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            } else {
                originalData = await response.json();

                avatar = originalData.avatar || null;
                username = originalData.username || null;
                name = originalData.name || null;
                phone = originalData.phone || null;
                address = originalData.address || null;
                email = originalData.email || null;
            }
        } catch (error) {
            console.error('There was a problem with the fetch operation: ', error);
        }
    });

    function handleFileChange(event: Event) {
        const target = event.target as HTMLInputElement;
        const file = target.files ? target.files[0] : null;

        if (file) {
            const reader = new FileReader();
            reader.onload = () => {
                avatar = reader.result as string;
            };

            reader.readAsDataURL(file);
        }
    }

    async function handleSubmit(event: Event) {
        event.preventDefault();

        const modifiedFields: any = {};

        if (avatar !== originalData.avatar) modifiedFields.avatar = avatar;
        if (username !== originalData.username) modifiedFields.username = username;
        if (name !== originalData.name) modifiedFields.name = name;
        if (phone !== originalData.phone) modifiedFields.phone = phone;
        if (address !== originalData.address) modifiedFields.address = address;
        if (email !== originalData.email) modifiedFields.email = email;

        if (Object.keys(modifiedFields).length == 0) return;

        try {
            await fetch('http://localhost:5443/api/users/me', {
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
        <input type="file" name="avatar" id="avatar" on:change={handleFileChange} />

        <input bind:value={username} type="text" name="username" id="username" placeholder="Apelido: " />
        <input bind:value={name} type="text" name="name" id="name" placeholder="Nome: " />
        <input bind:value={phone} type="number" name="phone" id="phone" placeholder="Número de celular: (63) 9 8129 4124" />
        <input bind:value={address} type="text" name="address" id="address" placeholder="Endereço: " />
        <input bind:value={email} type="email" name="email" id="email" placeholder="Email: " />

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
    border: 0px solid;
    border-radius: 0.5rem;
    transition: 150ms;
    cursor: pointer;
    outline: none;
}

input:hover {
    padding: 0.5rem;
    font-size:large;
}

input:focus {
    padding: 0.5rem;
    font-size:large;
    border-bottom: 1px solid hsl(120, 17%, 70%);
}

form > input, form > button{
    width: 100%;
}
</style>
