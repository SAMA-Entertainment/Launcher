<script setup>
import { reactive } from "vue";
import Button from "./components/Button.vue";
import UpdateModal from "./modals/UpdateModal.vue";
import SettingsModal from "./modals/SettingsModal.vue";
import { Command } from '@tauri-apps/api/shell'
import { process } from "@tauri-apps/api";

let state = reactive({
    loading: false,
    message: 'Jouer',
    checkingUpdate: false,
    showSettings: false,
});

function launchGame(){
    if(state.loading) return;
    state.loading = true;
    state.message = 'Démarrage';
    new Command('run-game', []).spawn().then(() => process.exit(0))
        .catch(e => state.message = 'Erreur: ' + e);
}

function checkUpdates(){
    if(state.loading) return;
    state.loading = true;
    state.checkingUpdate = true;
}

function showSettings(){
    if(state.loading) return;
    state.loading = true;
    state.showSettings = true;
}

function closeModal(){
    state.loading = false;
    state.showSettings = false;
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
                    <Button color="purple" :disabled="state.loading" @click="checkUpdates">Vérifier les mises à jour</Button>
                    <Button color="blue" :disabled="state.loading" @click="showSettings">Paramètres</Button>
                </div>
            </div>
        </div>
        <UpdateModal v-if="state.checkingUpdate" @close="closeModal" />
        <SettingsModal v-if="state.showSettings" @close="closeModal" />
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
    font-weight: 400;
    line-height: 1.5;
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
    background: url("./assets/Screenshot.png") no-repeat;
    background-size: cover;
    padding-top: 60px;
    height: 100%;
}

.launcher .logo {
    width: 500px;
    user-select: none;
}

.launcher .footer {
    background-color: white;
    border-radius: 100px 100px 0 0;
    box-shadow: 0 -3px 6px rgba(0, 0, 0, .1);
    padding: 20px 50px;
    width: 100%;
}

.launcher .footer .actions {
    display: flex;
    justify-content: center;
    flex-direction: column;
    gap: 14px;
    margin-top: -75px;
}

.launcher .more {
    display: flex;
    gap: 9px;
    justify-content: center;
}

.launcher .more a {
    color: #363232;
    text-decoration: none;
}

.launcher .more a:hover, .launcher .more a:focus {
    text-decoration: underline;
}

h1, h2, h3, h4, h5 {
    margin: 3rem 0 1.38rem;
    font-family: 'Luckiest Guy', sans-serif;
    font-weight: 400;
    line-height: 1.3;
}

h1 {
    margin-top: 0;
    font-size: 4.209rem;
}

h2 {font-size: 3.157rem;}

h3 {font-size: 2.369rem;}

h4 {font-size: 1.777rem;}

h5 {font-size: 1.333rem;}

small, .text_small {font-size: 0.75rem;}

h6 {
    color: rgba(54, 50, 50, 0.3);
    font-size: 24px;
}
</style>
