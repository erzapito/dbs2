<script>
import axios from 'axios';

import WantedItem from '../components/wanted-item.vue';

export default {
  name: 'WantedController',
  data: () => ({
    items: [],
  }),
  components: {
    WantedItem,
  },
  mounted: function(){
    this.loadCurrentPage();
  },
  methods: {
    loadCurrentPage: function() {
        axios
          .get('api/wanted')
          .then((response) => {
            this.items = response.data.items;
          });
    }
  }
}
</script>

<template>
  <div class="wanted-controller controller">
    <div class="title">Wanteds</div>

    <table class="listing">
        <tr class="headers">
            <th>Id</th>
            <th>Artist</th>
            <th>Disc</th>
            <th>Weeks</th>
            <th>Done</th>
            <th>Mark</th>
            <th>Downloaded</th>
        </tr>
        <wanted-item
            v-for="item in items"
            v-bind:key="item.id"
            :item="item"
            v-on:reload="loadCurrentPage()" />
    </table>
  </div>
</template>

<style lang="scss">
.wanted-controller {
    .listing {
        width: 100%;
        display: block;        

        tr.headers {
            background-color: #f2d2d2;
        }

        tr:nth-child(even) {
            background-color: #f2f2f2;
        }

        th, td {
            text-align: left;
            padding: 4px;
            border-bottom: 1px solid #ddd;
        }
    }
}
</style>
