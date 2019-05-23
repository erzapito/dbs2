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
    <paginator v-bind:page="5" v-bind:page-size="10" v-bind:total-items="100" />
  </div>
</template>

<script lang="ts">
import Vue from 'vue'
import Component from 'vue-class-component'
import axios from 'axios'

import Paginator from '../components/paginator.vue'
import SeriesResponse from '../api/SeriesResponse'

@Component({
    components: {
        'paginator': Paginator
    }
})
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
