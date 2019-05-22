<style>
</style>
<template>
  <div>
    <h2>Series</h2>
    <ul>
        <li v-for="item in info.items">
            {{item.name}}
        </li>
    </ul>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'
import Component from 'vue-class-component'
import axios from 'axios'

import SeriesResponse from '../api/SeriesResponse'

@Component
export default class SeriesCtrl extends Vue {
  message: string = 'Welcome to Your Vue.js App';
  currentPage: number = 0;
  info: SeriesResponse = new SeriesResponse();


  mounted() {
    this.loadCurrentPage();
  }

  loadCurrentPage() {
    axios
      .get<SeriesResponse>('/api/series')
      .then(response => (this.info = response.data))
  }

}
</script>
