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
          .get('/api/wanted')
          .then((response) => {
            this.items = response.data.items;
          });
    }
  }
}
</script>

<template>
  <div class="wanted-controller">
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
