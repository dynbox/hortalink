<script lang="ts">
    import google from "$lib/assets/google.svg";
    import facebook from "$lib/assets/facebook.svg";
    import linkedin from "$lib/assets/linkedin.svg";

    async function handleExternalLogin(oauthType: string) {
        try {
            const response = await fetch(`http://localhost:5443/api/v1/oauth/${oauthType}`, {
                method: 'POST',
                credentials: 'include'
            });

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            } else {
                let data = await response.json()

                window.location.href = data.auth_url;
            }
        } catch (error) {
            console.error('There was a problem with the fetch operation: ', error);
        }
    }
</script>

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