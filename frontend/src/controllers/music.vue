<script>
import axios from 'axios';

import Paginator from '../components/paginator.vue';
import MusicItem from '../components/music-item.vue';

export default {
  name: 'MusicController',
  data: () => ({
    'currentPage': 1,
    'newElement' : false,
    'info': {
        items: [],
        total: 0,
       }
  }),
  components: {
    Paginator,
    MusicItem,
  },
  mounted: function(){
    this.loadCurrentPage();
  },
  methods: {
    setPage: function(page) {
        this.currentPage = page;
        this.loadCurrentPage();
    },
    loadCurrentPage: function() {
        axios
          .get('/api/musica', {
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
    <h2>Musica</h2>

    <div>
        <a href="javascript:void(0)" v-on:click="newElement = !newElement">New</a>
        <music-form
            v-if="newElement"
            :item="newitem"
            v-on:series-saved="setPage(0)" />
    </div>

    <ul>
        <li v-for="item in info.items" v-bind:key="item.id" >
            <music-item
                :item="item"
                v-on:reload-series="loadCurrentPage()" />
        </li>
    </ul>
    <paginator
      v-bind:page="currentPage"
      v-bind:page-size="10"
      v-bind:total-items="info.total"
      v-on:paginator-page-change="setPage($event)" />
  </div>
</template>
