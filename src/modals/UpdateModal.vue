<template>
<Modal class="update-modal">
    <h1>Mise à jour</h1>
    <h6>{{ state.task }}</h6>
    <div class="space">
        <UpdateSpinner />
    </div>
    <Button color="red" @click="$emit('close')">Annuler</Button>
</Modal>
</template>

<script setup>
import Modal from "../components/Modal.vue";
import { reactive } from "vue";
import Button from "../components/Button.vue";
import UpdateSpinner from "../components/UpdateSpinner.vue";
import { fs, path, tauri, event } from "@tauri-apps/api";

const emit = defineEmits(['close'])
let state = reactive({
    task: 'Récupération des données...'
});


let updateInfo;
fetch('https://mikuni.me/auto_update_check.json')
    .then(r => {
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
        if(!result.update){
            state.task = 'À jour';
            return;
        }
        state.task = 'Préparation de la màj...';
        updateInfo = result;
        let localPath = await path.dataDir();
        localPath = await path.join(localPath, "Mikuni");

        let unlisten = await event.listen('update_progress', (e) => {
            state.task = 'Téléchargement de la màj... ' + e.payload + '%';
        });
        let unlisten2 = await event.listen('install_progress', (e) => {
            state.task = 'Copie des fichiers... ' + e.payload + '%';
        });
        await tauri.invoke('download_update', { dir: localPath, tag: result.tag }) // Call rust function
        unlisten();
        unlisten2();
        state.task = `Mise à jour terminée!`;
    })
    .catch(e => {
        state.task = `Erreur: ${e}`;
    });
</script>

<style>
.update-modal {
    color: #363232;
}
.update-modal .space {
    padding: 100px 300px;
}
</style>