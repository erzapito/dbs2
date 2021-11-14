<script>
export default {
    name: 'Paginator',
    props: ['page','pageSize','totalItems'],
   
    computed: {
        totalPages: function() {
            var v = Math.ceil(this.totalItems / this.pageSize) + 1;
            if (v && this.page >= v) {
              this.$emit('paginator-page-change',v)
            }
            return v;
        },
    },
}
</script>

<template>
    <div class="paginator" v-if="totalPages > 1">
        <span class="cell first" v-if="totalPages > 1 && page != 1" v-on:click="$emit('paginator-page-change',1)">
          <a href="javascript:void(0)">1</a>
        </span>
        <span class="cell separator left" v-if="page>3">...</span>
        <span class="cell previous" v-if="page>2" v-on:click="$emit('paginator-page-change',page - 1)">
          <a href="javascript:void(0)">{{ page - 1 }}</a>
        </span>
        <span class="cell current">{{ page }}</span>
        <span class="cell next" v-if="page<(totalPages-1)" v-on:click="$emit('paginator-page-change',page + 1)">
          <a href="javascript:void(0)">{{ page + 1 }}</a>
        </span>
        <span class="cell separator right" v-if="page<(totalPages-3)">...</span>
        <span class="cell last" v-if="(page != totalPages)" v-on:click="$emit('paginator-page-change',totalPages)">
          <a href="javascript:void(0)">{{ totalPages }}</a>
        </span>
    </div>
</template>

<style lang="scss">
.paginator {
    margin: 10px;
    display: flex;
    justify-content: center;

    .cell {
        display: flex;
        width: 40px;
        height: 40px;
        align-items: center;
        justify-content: center;
        border: 1px solid black;
    }
}
</style>
