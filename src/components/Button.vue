<script setup>
import { computed } from 'vue'

const props = defineProps({
    color: {
        type: String,
        default: 'primary',
        validator: function (value) {
            return ["green", "purple", "blue", "red", "gray"].indexOf(value) !== -1;
        },
    },
    disabled: {
        type: Boolean,
        default: false
    },
    size: {
        type: String,
        default: "medium",
        validator: function (value) {
            return ["medium", "large"].indexOf(value) !== -1;
        },
    },
    loading: {
        type: Boolean,
        default: false,
    }
})
const classes = computed(() => {
    return ['mk-button', `mk-button--${props.size}`, `mk-button--${props.color}`,
        props.loading ? 'mk-button--loading' : '' ]
});
</script>

<template>
    <button :class="classes" :disabled="disabled">
        <slot></slot>
    </button>
</template>

<style>
.mk-button {
    display: flex;
    justify-content: center;
    align-items: center;
    border-radius: 4px;
    padding: 12px 18px;
    border: none;
    transition: background-color 0.3s;
    background-color: #C4C4C4;
    color: #ffffff;
}

.mk-button:disabled {
    cursor: not-allowed;
}

.mk-button.mk-button--medium {
    font-size: 32px;
}

.mk-button.mk-button--large {
    font-size: 42px;
    text-shadow: 0 4px rgba(0, 0, 0, 0.16);
    padding: 32px 180px;
    box-shadow: 0 5px 6px rgba(0, 0, 0, 0.10);
}

.mk-button.mk-button--green {
    background-color: #91D55D;
}

.mk-button.mk-button--green:hover {
    background-color: #88c757;
}

.mk-button.mk-button--green:focus {
    box-shadow: 0 0 0 0.125em rgba(145 213 93 / 50%);
}

.mk-button.mk-button--purple {
    background-color: #B47FB8;
}

.mk-button.mk-button--purple:hover {
    background-color: #a575a8;
}

.mk-button.mk-button--purple:focus {
    box-shadow: 0 0 0 0.125em rgba(180 127 184 / 50%);
}

.mk-button.mk-button--blue {
    background-color: #73BBD2;
}

.mk-button.mk-button--blue:hover {
    background-color: #69aabf;
}

.mk-button.mk-button--blue:focus {
    box-shadow: 0 0 0 0.125em rgba(115 187 210 / 50%);
}

.mk-button.mk-button--red {
    background-color: #D55D5D;
}

.mk-button.mk-button--red:hover {
    background-color: #c25555;
}

.mk-button.mk-button--red:focus {
    box-shadow: 0 0 0 0.125em rgba(213 93 93 / 50%);
}

.mk-button.mk-button--gray:hover {
    background-color: #bbbbbb;
}

.mk-button.mk-button--gray:focus {
    box-shadow: 0 0 0 0.125em rgba(196 196 196 / 50%);
}
</style>