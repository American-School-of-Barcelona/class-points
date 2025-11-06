<script setup>
  import { ref, onMounted } from "vue";

  const leaderboard = ref([])

  onMounted(() => {
    leaderboard.value = [
      { id: 1, username: "AlphaWolf", points: 98.56 },
      { id: 2, username: "PixelPanda", points: 92.43 },
      { id: 3, username: "CodeNinja", points: 88.75 },
      { id: 4, username: "DataDragon", points: 85.29 },
      { id: 5, username: "ByteMaster", points: 80.05 },
      { id: 5, username: "ByteMaster", points: 80.05 },
      { id: 5, username: "ByteMaster", points: 80.05 },
      { id: 5, username: "ByteMaster", points: 80.05 },
      { id: 5, username: "ByteMaster", points: 80.05 },
      { id: 5, username: "ByteMaster", points: 80.05 },
      { id: 5, username: "ByteMaster", points: 80.05 },
      { id: 5, username: "ByteMaster", points: 80.05 },
      { id: 5, username: "ByteMaster", points: 80.05 },
      { id: 5, username: "ByteMaster", points: 80.05 },
      { id: 5, username: "ByteMaster", points: 80.05 },
      { id: 5, username: "ByteMaster", points: 80.05 },
      { id: 5, username: "ByteMaster", points: 80.05 },
      { id: 5, username: "ByteMaster", points: 80.05 },
    ];
  });
</script>

<template>
    <div class="widget-card">
      <table v-if="leaderboard.length > 0" class="styled-table" style="border-radius: 7px;">
        <thead>
        <tr>
          <th>#</th>
          <th>Username</th>
          <th>Point</th>
        </tr>
        </thead>
        <tbody>
        <tr v-for="(entry, idx) in leaderboard" :key="entry.username">
          <td>{{ idx + 1 }}</td>
          <td>{{ entry.username }}<span style="font-size: 0.7em; color: dimgrey"><i>&ensp;({{ entry.id }})</i></span></td>
          <td>{{ entry.points.toFixed(2) }}</td>
        </tr>
        </tbody>
      </table>
      <span v-else>No one here...</span>
    </div>
</template>

<style scoped>
.styled-table {
  width: 100%;
  border-collapse: collapse;
  background: rgba(255,255,255,0.05);
  border-radius: 12px;
  overflow: hidden;
  color: var(--color-text);
  font-family: 'Inter', sans-serif;
}

.styled-table th,
.styled-table td {
  padding: 10px 12px;
  text-align: left;
}

.styled-table th {
  background: rgba(255,255,255,0.1);
  font-weight: 600;
}

.styled-table tr {
  transition: background 0.3s ease;
}

.styled-table tr:hover {
  background: rgba(255,255,255,0.1);
}
</style>