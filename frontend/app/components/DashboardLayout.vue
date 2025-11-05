<script setup>
import List from './List.vue'
</script>

<template>
    <div class="app">
        <div class="container">
            <div class="header">
                <h1>Dashboard</h1>
            </div>

            <List></List>

        </div>
    </div>
</template>

<style>
body {
    margin: 0;
    background-color: var(--color-bg);
    font-family: 'Inter', sans-serif;
    height: 100vh;
}

.app {
    height: 100vh;
    background-color: var(--color-bg);
    display: flex;
    justify-content: center;
    align-items: stretch;
}

.container {
    display: flex;
    flex-direction: column;
    width: 100vw;
    height: calc(100vh - 4em);
    border-radius: calc(10px + 2em);
    margin: 2em;
    padding: 2em;
    background-color: var(--color-bg-alt);
    box-sizing: border-box;
}

.header {
    margin-bottom: 1rem;
    color: var(--color-text);
}

.header h1 {
    margin: 0;
    font-size: 1.8rem;
    color: var(--color-accent);
}

.grid {
    flex: 1;
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    grid-auto-rows: 1fr;
    gap: 1rem;
    width: 100%;
}

.grid .wide {
    grid-column: span 2;
}
</style>