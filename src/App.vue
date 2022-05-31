<script setup>
import Button from "./components/Button.vue";
import { reactive } from "vue";
import UninstallModal from "./modals/UninstallModal.vue";
import UpdateModal from "./modals/UpdateModal.vue";

let state = reactive({
    loading: false,
    message: 'Jouer',
    checkingUpdate: false,
    confirmUninstall: false
});

function launchGame(){
    if(state.loading) return;
    state.loading = true;
    state.message = 'Démarrage';
}

function checkUpdates(){
    if(state.loading) return;
    state.loading = true;
    state.checkingUpdate = true;
}

function uninstall(){
    if(state.loading) return;
    state.loading = true;
    state.confirmUninstall = true;
}

function closeModal(){
    state.loading = false;
    state.confirmUninstall = false;
    state.checkingUpdate = false;
}
</script>

<template>
    <div class="launcher">
        <img alt="Mikuni Logo" src="./assets/mikuni_logo.png" class="logo" />
        <div class="footer">
            <div class="actions">
                <div class="more">
                    <Button color="green" size="large" :disabled="state.loading" :loading="state.loading"
                    @click="launchGame">{{ state.message }}</Button>
                </div>
                <div class="more">
                    <Button color="purple" :disabled="state.loading" @click="checkUpdates">Vérifier les mise à jour</Button>
                    <Button color="red" :disabled="state.loading" @click="uninstall">Désinstaller</Button>
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
        <UpdateModal v-if="state.checkingUpdate" @close="closeModal" />
        <UninstallModal v-if="state.confirmUninstall" @close="closeModal" />
    </div>
</template>

<style>
@font-face {
    font-family: "Luckiest Guy";
    src: url("./assets/LuckiestGuy-Regular.ttf") format("truetype");
}

body {
    width: 100vw;
    height: 100vh;
}

* {
    padding: 0;
    margin: 0;
    box-sizing: border-box;
    font-family: "Luckiest Guy", sans-serif;
}

#app {
    color: #333333;
    width: 100%;
    height: 100%;
}

.launcher {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
    background: url("./assets/Screenshot.png");
    padding-top: 60px;
    height: 100%;
}

.launcher .logo {
    width: 500px;
}

.launcher .footer {
    background-color: white;
    border-radius: 100px 100px 0 0;
    box-shadow: 0 -3px 6px rgba(0, 0, 0, .1);
    padding: 0 50px;
    width: 100%;
}

.launcher .footer .actions {
    display: flex;
    justify-content: center;
    flex-direction: column;
    gap: 14px;
    margin-top: -55px;
}

.launcher .actions .more {
    display: flex;
    gap: 9px;
    justify-content: center;
}

.launcher .infos {
    display: flex;
    flex-direction: column;
}

.launcher .infos .info {
    display: flex;
    gap: 8px;
    opacity: 0.3;
}

h1 {
    font-size: 64px;
}

h6 {
    color: rgba(54, 50, 50, 0.3);
    font-size: 24px;
}
</style>
