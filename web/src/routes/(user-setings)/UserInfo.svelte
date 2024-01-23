<script>
// @ts-nocheck

        import google from '$lib/assets/google.svg';

    let fakeAPI = {
        fotoPerfil: google,
        apelido: "zé da manga",
        nome: "alyson guilherme teixeira de oliveira",
        numero: "63981294124",
        endereco: "605n chacara agua fria 39",
        email: "alysontxr@gmail.com",
        senha: "Leleco23@"
    }

    
    function handleFileChange(event) {
    const file = event.target.files[0];

    if (file) {
      const reader = new FileReader();
      reader.onload = () => {
        fakeAPI.fotoPerfil = reader.result;
      };
      reader.readAsDataURL(file);
    }
  }

</script>

<div class="component-elements">
    <form action="">
        
        <img src={fakeAPI.fotoPerfil} alt="">
        <label for="foto-perfil">clique para trocar foto</label>
        <input type="file" name="foto-perfil" id="foto-perfil" on:change={handleFileChange} />

        <input bind:value={fakeAPI.apelido} type="text" name="apelido" id="apelido" placeholder="seu apelido: " />
        <input bind:value={fakeAPI.nome} type="text" name="nome" id="nome" placeholder="seu nome: " />
        <input bind:value={fakeAPI.numero} type="number" name="numero" id="numero" placeholder="seu numero de celular: (63) 9 8129 4124" />
        <input bind:value={fakeAPI.endereco} type="text" name="endereco" id="endereco" placeholder="seu endereço: " />
        <input bind:value={fakeAPI.email} type="email" name="email" id="email" placeholder="seu email: " />
        <input bind:value={fakeAPI.senha} type="password" name="senha" id="senha" placeholder="sua senha: ">
    
        <button type="submit" value="salvar">
            salvar
        </button>
    </form>
</div>

<style>
.component-elements{
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

img{
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
