<style>
</style>

<template>
    <div>
        <span v-if="page != 1">1</span>
        <span v-if="page>3">...</span>
        <span v-if="page>2">{{ page - 1 }}</span>
        <span>{{ page }}</span>
        <span v-if="page<(totalPages-1)">{{ page + 1 }}</span>
        <span v-if="page<(totalPages-3)">...</span>
        <span v-if="(page != totalPages)">{{ totalPages }}</span>
    </div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';

@Component({
    name: 'paginator',
})
export default class Paginator extends Vue {

  @Prop({default: 1})
  public page: number;

  @Prop({default: 100})
  public totalItems: number;

  @Prop({default: 10})
  public pageSize: number;

  get totalPages() {
     return Math.ceil( this.totalItems / this.pageSize );
  }

}
</script>
