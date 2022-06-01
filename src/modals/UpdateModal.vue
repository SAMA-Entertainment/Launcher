<template>
<Modal class="update-modal">
    <h1>Mise à jour</h1>
    <h6>{{ state.task }}</h6>
    <div class="space">
        <UpdateSpinner v-if="state.resultState === 0" />
        <img v-else-if="state.resultState === 1" src="../assets/alert.svg" alt="Error">
        <img v-else-if="state.resultState === 2" src="../assets/check.svg" alt="Success">
    </div>
    <Button color="red" @click="tryAbort" :disabled="state.installing">{{ cancelText }}</Button>
</Modal>
</template>

<script setup>
import Modal from "../components/Modal.vue";
import { reactive, computed } from "vue";
import Button from "../components/Button.vue";
import UpdateSpinner from "../components/UpdateSpinner.vue";
import { fs, path, tauri, event } from "@tauri-apps/api";

const emit = defineEmits(['close'])
let state = reactive({
    task: 'Récupération des données...',
    installing: false,
    resultState: 0,
    abort: false
});

function tryAbort(){
    if(state.installing) return;
    state.abort = true;
    emit('close');
}

const cancelText = computed(() => ["Annuler", "Fermer", "Terminer"][state.resultState]);

let updateInfo;
fetch('https://mikuni.me/auto_update_check.json')
    .then(r => {
        if(state.abort){
            throw new Error("Mise à jour annulée");
        }
        if(!r.ok){
            throw new Error("Failed to reach update server (" + r.status + ")");
        }
        return r.json()
    })
    .then(async remoteConfig => {
        state.task = 'Vérification des fichiers';

        let localConfigPath = await path.dataDir();
        localConfigPath = await path.join(localConfigPath, "Mikuni", "Game", "MikuniGame", "version.json");
        let localConfig = await fs.readTextFile(localConfigPath);
        localConfig = JSON.parse(localConfig);
        if(remoteConfig.version_number > localConfig.version_number){
            return Object.assign({}, remoteConfig, { update: true })
        }
        return Object.assign({}, localConfig, { update: false })
    })
    .then(async result => {
        if(state.abort){
            throw new Error("Mise à jour annulée");
        }
        if(!result.update){
            state.task = 'À jour';
            state.resultState = 2;
            return;
        }
        state.task = 'Préparation de la màj...';
        state.installing = true;
        updateInfo = result;
        let localPath = await path.dataDir();
        localPath = await path.join(localPath, "Mikuni");

        let unlisten_progress = await event.listen('update_progress', (e) => {
            state.task = 'Téléchargement de la màj... ' + e.payload + '%';
        });
        let unlisten_install = await event.listen('install_progress', (e) => {
            state.task = 'Copie des fichiers... ' + e.payload + '%';
        });
        await tauri.invoke('download_update', { dir: localPath, tag: result.tag }) // Call rust function
        unlisten_progress();
        unlisten_install();
        state.task = `Mise à jour terminée!`;
        state.installing = false;
        state.resultState = 2;
    })
    .catch(e => {
        state.resultState = 1;
        state.task = `Erreur: ${e}`;
    });
</script>

<style>
.update-modal {
    color: #363232;
}
.update-modal .space {
    padding: 15vh 25vw;
}
.update-modal .space img {
    width: 96px;
    height: 96px;
}
</style>