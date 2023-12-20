
<style>
    *{
        box-sizing: border-box;
        margin: 0;
        padding: 0;
    }
    main{
        position: absolute;
        left: 0;
        top: 0;
        width: 100vw; 
        height: 100vh;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
    }
    form{
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        width: 100%;
        max-width: 425px;
        padding: 1rem;
    }
    form > input{
        width: 100%;
        margin: 0.5rem 0.5rem 0 0.5rem;
        padding: 0.25rem;
    }
    .user-type-container{
        display:flex;
        justify-content: space-around;
        width: 100%;
    }

    .user-type, button{
        margin: 0.5rem 0.5rem 0 0.5rem;
        padding: 0.25rem;
        border-radius: 0.25rem;
        transition: 1s;
    }
    .user-active{
        background-color: #228b22;
        color: #fff;
    }

</style>

<main>
    <h2>bem vindo ao:</h2>
    <h1>hortalink</h1>

    <form action="">
        <p>entre agora!</p>
        <input type="email" placeholder="e-mail:" />
        <input type="password" placeholder="senha:" />
        <div class="user-type-container">
            <span class="user-type user-active">cliente</span>
            <span class="user-type">vendedor</span>
        </div>

        <button type="submit">enviar</button>
    </form>
    <p>não tem uma conta? <a href="">inscreva-se aqui</a></p>
</main>