<script>
import axios from 'axios';

import Paginator from '../components/paginator.vue';
import MusicItem from '../components/music-item.vue';
import MusicForm from '../components/music-form.vue';

export default {
  name: 'MusicController',
  data: () => ({
    'currentPage': 1,
    'search' : '',
    'newElement' : false,
    'info': {
        items: [],
        total: 0,
       }
  }),
  components: {
    Paginator,
    MusicItem,
    MusicForm
  },
  mounted: function(){
    this.loadCurrentPage();
  },
  methods: {
    delayedLoad: function() {
        if (this.delayedLoadVar) {
            clearTimeout(this.delayedLoadVar);
        }
        this.delayedLoadVar = setTimeout( this.loadCurrentPage, 500);
    },
    setPage: function(page) {
        this.currentPage = page;
        this.loadCurrentPage();
    },
    loadCurrentPage: function() {
        this.newElement = false;
        this.newItem = {};
        axios
          .get('/api/music', {
                params : {
                    page : this.currentPage - 1,
                    search : this.search,
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
  <div class="music-controller">
    <div class="title">Musica</div>

    <div class="search">
        <input type="text" v-model="search" v-on:keyup="delayedLoad()" />
    </div>

    <div class="new">
        <a href="javascript:void(0)" v-on:click="newElement = !newElement">New</a>
        <music-form
            v-if="newElement"
            :item="newItem"
            v-on:series-saved="setPage(0)" />
    </div>

    <ul class="listing">
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
