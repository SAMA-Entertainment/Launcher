<script setup>
import Button from "./components/Button.vue";
import Spinner from "./components/Spinner.vue";
</script>

<template>
    <img alt="Mikuni Logo" src="./assets/mikuni_logo.png"/>
    <Spinner />
    <div class="footer">
        <div class="actions">
            <Button>Jouer</Button>
            <div class="more">
                <Button small>Vérifier les mise à jour</Button>
                <Button small>Désinstaller</Button>
            </div>
        </div>
        <div class="infos">
            <div class="info">
                <p>Version du launcher: {}</p>
                <p>Version du jeu: {}</p>
            </div>
            <div class="info">
                <p>Dossier d'installation: {}</p>
            </div>
        </div>
    </div>
</template>

<style>
@font-face {
    font-family: "Luckiest Guy";
    src: url("./assets/LuckiestGuy-Regular.ttf") format("truetype");
}

* {
    padding: 0;
    margin: 0;
    box-sizing: border-box;
}

#app {
  font-family: "Luckiest Guy", sans-serif;
  color: #333333;
}
</style>
