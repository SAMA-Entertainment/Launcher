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
import Spinner from "../components/Spinner.vue";
import Button from "../components/Button.vue";
import UpdateSpinner from "../components/UpdateSpinner.vue";

let state = reactive({
    task: 'Récupération des données...'
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