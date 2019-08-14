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
    <paginator
        v-bind:page="currentPage"
        v-bind:page-size="10"
        v-bind:total-items="info.total"
        v-on:paginator-page-change="setPage($event)"/>
  </div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import axios from 'axios';

import Paginator from '../components/paginator.vue';
import SeriesResponse from '../api/SeriesResponse';

@Component({
    components: {
        paginator: Paginator,
    },
})
export default class SeriesCtrl extends Vue {

  private currentPage: number = 1;
  private info: SeriesResponse = new SeriesResponse();

  public mounted() {
    this.loadCurrentPage();
  }

  public setPage(page: number) {
    this.currentPage = page;
    this.loadCurrentPage();
  }

  public loadCurrentPage() {
    axios
      .get<SeriesResponse>('/api/series', {
            params : {
                page : this.currentPage - 1,
            },
        })
      .then((response) => {
        this.info = response.data;
      });
  }

}
</script>
