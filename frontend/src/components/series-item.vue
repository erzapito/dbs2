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
    <div class="item">
        <div class="content">
            <span class="id">{{ item.id }}</span>
            <a class="toggle" href="javascript:void(0)" v-on:click="editing = !editing">{{ item.name }}</a>
        </div>
        <series-form
            class="edit"
            v-if="editing"
            :item="item"
            v-on:series-saved="$emit('reload-series')"
            v-on:series-deleted="$emit('reload-series')" />
    </div>
</template>
