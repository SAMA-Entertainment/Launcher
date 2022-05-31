<template>
<div class="modal">
    <div class="modal--body">
        <slot></slot>
    </div>
</div>
</template>

<script>
export default {
    name: "Modal"
}
</script>

<style>
.modal {
    display: flex;
    justify-content: center;
    align-items: center;
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0,0,0,.16);
}
.modal .modal--body {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    padding: 40px;
    background-color: #FFFFFF;
    border-radius: 8px;
}
</style>