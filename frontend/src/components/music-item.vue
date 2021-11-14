<script>
import MusicForm from './music-form.vue';

export default {
    props: [ 'item' ],
    data: function(){
        return {
            'editing': false,
        };
    },
    components: {
        MusicForm,
    },
    methods: {}
}
</script>

<template>
    <div class="item">
        <div class="content">
            <span class="id">{{ item.id }}</span>
            <a class="toggle" href="javascript:void(0)" v-on:click="editing = !editing">{{ item.artist }} - {{ item.disc }}</a>
        </div>

        <music-form
          v-if="editing"
          @close="editing=false"
          :item="item"
          v-on:music-saved="$emit('reload-music')"
          v-on:music-deleted="$emit('reload-music')" />
    </div>
</template>
