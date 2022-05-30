<script setup>
defineProps({
    small: { type: Boolean }
})
</script>

<template>
    <button>
        <slot></slot>
    </button>
</template>

<style scoped>
button {
    border-radius: 8px;
    padding: 12px 16px;
    border: none;
}
</style>