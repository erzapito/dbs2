<script>
import SeriesForm from './series-form.vue';

export default {
    props: [ 'item' ],
    data: function(){
        return {
            'editing': false,
        };
    },
    components: {
        SeriesForm,
    },
    methods: {}
}
</script>

<template>
    <div class="series-item">
        <span class="id">{{ item.id }}</span>
        <a class="toggle" href="javascript:void(0)" v-on:click="editing = !editing">{{ item.name }}</a>
        <series-form
            v-if="editing"
            :item="item"
            v-on:series-saved="$emit('reload-series')"
            v-on:series-deleted="$emit('reload-series')" />
    </div>
</template>

<style lang="scss">
.series-item {
    display: inline-block;
}

</style>
