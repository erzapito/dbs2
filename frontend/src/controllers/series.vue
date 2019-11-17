<script>
import axios from 'axios';

import Paginator from '../components/paginator.vue';
import SeriesItem from '../components/series-item.vue';

export default {
  name: 'SeriesController',
  data: () => ({
    'currentPage': 1,
    'info': {
        items: [],
        total: 0,
       }
  }),
  components: {
    Paginator,
    SeriesItem,
  },
  mounted: function(){
    this.loadCurrentPage();
  },
  methods: {
    setPage: function(page) {
        this.currentPage = page;
        this.loadCurrentPage();
    },
    loadCurrentPage: function(){
        axios
          .get('/api/series', {
                params : {
                    page : this.currentPage - 1,
                },
            })
          .then((response) => {
            this.info.items = response.data.items;
            this.info.total = response.data.total;
          });
    }
  }
}
</script>

<template>
  <div>
    <h2>Series</h2>
    <ul>
        <li v-for="item in info.items" v-bind:key="item.id" >
            <series-item :item="item" />
        </li>
    </ul>
    <paginator
      v-bind:page="currentPage"
      v-bind:page-size="10"
      v-bind:total-items="info.total"
      v-on:paginator-page-change="setPage($event)" />
  </div>
</template>
