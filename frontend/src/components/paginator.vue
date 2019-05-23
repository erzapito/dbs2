<style>
</style>

<template>
    <div>
        <span v-if="page != 1">1</span>
        <span v-if="page>3">...</span>
        <span v-if="page>2">{{ page - 1 }}</span>
        <span>{{ page }}</span>
        <span v-if="page<(totalPages-2)">{{ page + 1 }}</span>
        <span v-if="page<(totalPages-3)">...</span>
        <span v-if="(page != totalPages)">{{ totalPages }}</span>
    </div>
</template>

<script lang="ts">
import Vue from 'vue'
import { Component, Prop } from 'vue-property-decorator'

@Component({
    name: 'paginator'
})
export default class Paginator extends Vue {

  @Prop({default: 1})
  page: number = 1;

  @Prop({default: 100})
  totalItems: number = 100;

  @Prop({default: 10})
  pageSize: number = 10;

  get totalPages() {
     return Math.ceil( this.totalItems / this.pageSize );
  }

  mounted() {

  }

  loadCurrentPage() {
  }

}

</script>
