<script setup>
defineProps({
    title: String
})
</script>

<template>
    <div class="widget-card">
        <h2 class="widget-title">{{ title }}</h2>
        <div class="widget-body">
            <slot />
        </div>
    </div>
</template>

<style scoped>
.widget-card {
    background-color: var(--color-surface);
    border-radius: 10px;
    padding: 1rem;
    color: var(--color-text);
    display: flex;
    flex-direction: column;
}

.widget-title {
    font-size: 1.5rem;
    color: var(--color-text);
    margin: 0;
}

.widget-body {
    font-size: 0.95rem;
    line-height: 1.4;
}
</style>