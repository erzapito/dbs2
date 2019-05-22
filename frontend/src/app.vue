<template>
  <div id="app">
        <ul>
          <li><router-link to="/series">Series</router-link></li>
          <li><router-link to="/music">Music</router-link></li>
          <li><router-link to="/books">Books</router-link></li>
          <li><router-link to="/wanted">Wanted</router-link></li>
        </ul>
      <router-view />
  </div>
</template>

<script lang="ts">
import Vue from "vue";

export default {
    name: 'app'
};
</script>
