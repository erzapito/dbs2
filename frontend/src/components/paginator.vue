<style>

.paginator-cell{
  width: 20px;
  height: 20px;
  background: #CCC;
  margin: 5px;
}

</style>

<template>
    <div class="paginator" v-if="totalPages > 1">
        <span class="paginator-cell" v-if="totalPages > 1 && page != 1" v-on:click="$emit('paginator-page-change',1)">
          <a href="javascript:void(0)">1</a>
        </span>
        <span v-if="page>3">...</span>
        <span class="paginator-cell" v-if="page>2" v-on:click="$emit('paginator-page-change',page - 1)">
          <a href="javascript:void(0)">{{ page - 1 }}</a>
        </span>
        <span class="paginator-cell">{{ page }}</span>
        <span class="paginator-cell" v-if="page<(totalPages-1)" v-on:click="$emit('paginator-page-change',page + 1)">
          <a href="javascript:void(0)">{{ page + 1 }}</a>
        </span>
        <span v-if="page<(totalPages-3)">...</span>
        <span class="paginator-cell" v-if="(page != totalPages)" v-on:click="$emit('paginator-page-change',totalPages)">
          <a href="javascript:void(0)">{{ totalPages }}</a>
        </span>
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

  @Prop({default: 0})
  public totalItems: number;

  @Prop({default: 10})
  public pageSize: number;

  get totalPages() {
     return Math.ceil( this.totalItems / this.pageSize );
  }

}
</script>
