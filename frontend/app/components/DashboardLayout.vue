<script setup>
import List from './List.vue'
</script>

<template>
    <div class="app">
        <div class="container">
            <div class="header">
                <h1>Dashboard</h1>
            </div>
            <div class="scroll-wrapper">
              <List class="list"></List>
            </div>
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
.scroll-wrapper {
  overflow: hidden;
}

.list {
  overflow-y: scroll;
  border-radius: 10px;
  height: 100%;
  padding-right: 10px;
  box-sizing: content-box;
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

::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-track {
  background: var(--color-bg-alt);
  border-radius: 10px;
}

::-webkit-scrollbar-thumb {
  background: var(--color-accent);
  border-radius: 10px;
  transition: all 0.3s ease;
}

::-webkit-scrollbar-thumb:hover {
  background: var(--color-accent-hover);
}
</style>