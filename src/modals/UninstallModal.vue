<template>
<Modal class="uninstall-modal">
    <h1>Désinstaller</h1>
    <p>
        Êtes vous sur de vouloir désintaller le jeu Mikuni ainsi que tout ses fichiers ?
        Le launcher se fermera tout seul et sera également supprimé de cet ordinateur.
    </p>
    <div class="actions">
        <Button color="gray" @click="$emit('close')">Annuler</Button>
        <Button color="red">Désinstaller</Button>
    </div>
</Modal>
</template>

<script setup>
import Modal from "../components/Modal.vue";
import Button from "../components/Button.vue";
</script>

<style>
.uninstall-modal .modal--body {
    gap: 32px;
    max-width: 50vw;
}
.uninstall-modal .modal--body p {
    font-size: 22px;
    color: rgba(54, 50, 50, 0.5);
    text-align: center;
}
.uninstall-modal .actions {
    display: flex;
    flex-direction: row;
    gap: 8px;
}
</style>