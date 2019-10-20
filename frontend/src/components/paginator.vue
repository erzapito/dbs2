<script>
export default {
    name: 'Paginator',
    props: ['page','pageSize','totalItems'],
   
    computed: {
        totalPages: function() {
            return Math.ceil(this.totalItems / this.pageSize);
        },
    },
}
</script>

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
