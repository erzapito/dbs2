<style>
</style>
<template>
  <div>
    <h2>Series</h2>
  </div>
</template>
<script lang="ts">
import Vue from 'vue'
import Component from 'vue-class-component'

@Component
export default class SeriesCtrl extends Vue {
  message: string = 'Welcome to Your Vue.js App';
  currentPage: int = 0;
  /*name: 'series',
  data () {
    return {
      message: 'Welcome to Your Vue.js App'
    }
  },*/
  get fullMessage(): string {
      return `${this.message} from Typescript`;
  };
  /*created() {
    console.log('created');
  },
  methods: {*/
    clicked(){
      console.log('clicked');
    };
  /*}*/

  mounted() {
    loadCurrentPage();
  }

  loadCurrentPage() {
    axios
      .get('https://api.coindesk.com/v1/bpi/currentprice.json')
      .then(response => (this.info = response))
  }

}
</script>
