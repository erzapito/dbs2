<script>
import axios from 'axios';

import Paginator from '../components/paginator.vue';
import SeriesItem from '../components/series-item.vue';
import SeriesForm from '../components/series-form.vue';

export default {
  name: 'SeriesController',
  data: () => ({
    'currentPage': 1,
    'search' : '',
    'newElement' : false,
    'newItem': {},
    'info': {
        items: [],
        total: 0,
       }
  }),
  components: {
    Paginator,
    SeriesItem,
    SeriesForm,
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
        this.newItem = {
          "name": "",
          "capitulos": "",
          "categoria": "",
          "fansub": "",
          "idioma": "",
        };
        this.newElement = false;
        axios
          .get('api/series', {
                params : {
                    page : this.currentPage - 1,
                    search : this.search,
                },
            })
          .then((response) => {
            this.info.items = response.data.items;
            this.info.total = response.data.total;
          
            this.maxPage = Math.floor(response.data.total / 10) + 1;
            if (this.maxPage < this.currentPage) {
              this.setPage(this.maxPage);
            }
          });
    }
  }
}
</script>

<template>
  <div class="series-controller controller">
    
    <div class="page-header">
      <div class="search">
          <input type="text" v-model="search" v-on:keyup="delayedLoad()" placeholder="search" />
      </div>
      <div class="new">
        <button v-on:click="newElement = !newElement">+</button>
      </div>
    </div>

    <series-form
        v-if="newElement"
        @close="newElement=false"
        :item="newItem"
        v-on:series-saved="setPage(1)" />

    <ul class="listing">
        <li v-for="item in info.items" v-bind:key="item.id" >
            <series-item
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

<style lang="scss">
@import '../assets/controller.scss';
</style>
