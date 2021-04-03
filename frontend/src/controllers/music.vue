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
          .get('api/music', {
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
  <div class="music-controller controller">
    <div class="page-header">
      <div class="search">
        <input type="text" v-model="search" v-on:keyup="delayedLoad()" placeholder="Search" />
      </div>

      <div class="new">
        <button v-on:click="newElement = !newElement">+</button>
      </div>
    </div>

    <music-form
      v-if="newElement"
      @close="newElement=false"
      :item="newItem"
      v-on:music-saved="setPage(0)" />

    <ul class="listing">
        <li v-for="item in info.items" v-bind:key="item.id" >
            <music-item
                :item="item"
                v-on:reload-music="loadCurrentPage()" />
        </li>
    </ul>
    <paginator
      v-bind:page="currentPage"
      v-bind:page-size="10"
      v-bind:total-items="info.total"
      v-on:paginator-page-change="setPage($event)" />
  </div>
</template>

<style lang="scss">
@import '../assets/controller.scss';
</style>
