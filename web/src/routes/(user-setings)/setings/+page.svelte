<script>
    import UserInfo from "../UserInfo.svelte";


</script>

<div class="fader">

    <UserInfo />
    <a href="">configuracoes</a>
    <a href="">idioma</a>
</div>

<style>
    .fader{
        padding: 1rem;
    }
    a {
        padding: 1rem;
        margin-top: 1rem;
        width: 100%;
        max-width: 425px;
        border-radius: 0.5rem;
        border: 1px solid hsl(120, 17%, 70%);
        box-shadow: rgba(79, 161, 79, 0.2) 0px 2px 8px 0px;
    }
</style>