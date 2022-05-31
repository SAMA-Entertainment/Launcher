<template>
<Modal class="settings-modal">
    <h1>Paramètres</h1>
    <div class="infos">
        <div class="info">
            <span>Dossier d'installation:</span>
            <p>{{ state.installPath }}</p>
            <Button color="green" size="small" v-if="state.isInstallPathValid" @click="openInstallFolder">Ouvrir</Button>
        </div>
        <div class="info">
            <span>Version du Launcher:</span>
            <p>{{ state.launcherVersion }}</p>
        </div>
        <div class="info">
            <span>Version du jeu:</span>
            <p>{{ state.gameVersion }}</p>
        </div>
        <div class="info">
            <span>Dernière mise à jour:</span>
            <p>1 janvier 1970</p>
        </div>
    </div>

    <div class="actions">
        <Button color="gray" @click="$emit('close')">Fermer</Button>
    </div>
</Modal>
</template>

<script setup>
import Modal from "../components/Modal.vue";
import Button from "../components/Button.vue";
import { reactive } from "vue";
import { app, fs, path, os } from "@tauri-apps/api";
import { Command } from '@tauri-apps/api/shell'

let state = reactive({
    launcherVersion: 'Chargement...',
    gameVersion: 'Chargement...',
    installPath: '',
    isInstallPathValid: false
});
app.getVersion().then(v => state.launcherVersion = v).catch(e => state.launcherVersion = `{${e}}`);
path.resolve('.')
    .then(v => {
        state.installPath = v;
        return os.type()
    })
    .then(platform => state.isInstallPathValid = platform === "Windows_NT")
    .catch(e => state.installPath = `{${ e }}`);

function openInstallFolder(){
    if(!state.isInstallPathValid) return;
    new Command('open-folder', ['.']).spawn();
}

function openGameFolder(){
    if(!state.isGamePathValid) return;
    new Command('open-folder', [state.gamePath]).spawn();
}

</script>

<style>
.settings-modal .modal--body {
    gap: 32px;
    max-width: 50vw;
}
.settings-modal .modal--body p {
    font-size: 22px;
    color: rgba(54, 50, 50, 0.5);
    text-align: center;
}
.settings-modal .actions {
    display: flex;
    flex-direction: row;
    gap: 8px;
}
.settings-modal .infos {
    display: flex;
    flex-direction: column;
    gap: 8px;
}
.settings-modal .infos .info {
    display: flex;
    flex-flow: row wrap;
    gap: 6px;
    align-items: center;
}
.settings-modal .infos .info span {
    font-size: 22px;
}
.settings-modal .infos .info p {
    text-align: start;
}
</style>