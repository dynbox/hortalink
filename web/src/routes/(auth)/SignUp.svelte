<script lang="ts">

</script>


<form method="post" action="http://localhost:5443/sign">
    <input name="email" type="email" placeholder="E-mail:" />
    <input name="phone" type="number" placeholder="Número de telefone:" />
    <input name="password" type="password" placeholder="Senha:" />
    <label for="account-type">Tipo de conta:</label>

    <div class="account-type">
        <label for="cliente">
            <input type="radio" name="account-type" id="cliente" />
            Cliente
        </label>

        <label for="vendedor">
            <input type="radio" name="account-type" id="vendedor" />
            Vendedor
        </label>
    </div>
    <button type="submit">Cadastrar</button>
</form>
