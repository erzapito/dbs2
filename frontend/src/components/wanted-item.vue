<script>
import axios from 'axios';

export default {
    props: [ 'item' ],
    methods: {
        mark: function() {
            axios
                .post('/api/wanted/' + this.item.id + '/mark')
                .then(() => {
                    this.$emit('reload');
                });
        },
        downloaded: function() {
            axios
                .post('/api/wanted/' + this.item.id + '/downloaded')
                .then(() => {
                    this.$emit('reload');
                });
        },
    }
}
</script>

<template>
    <tr class="wanted-item">
        <td class="id">{{ item.id }}</td>
        <td class="artist">{{ item.artist }}</td>
        <td class="disc">{{ item.disc }}</td>
        <td class="weeks">{{ item.weeks }}</td>
        <td class="done">{{ item.done }}</td>
        <td class="mark">
            <a href="javascript:void(0)" v-on:click="mark()">Mark</a>
        </td>
        <td class="downloaded">
            <a href="javascript:void(0)" v-on:click="downloaded()">Downloaded</a>
        </td>
    </tr>
</template>

<style lang="scss">
.wanted-item {
    .artist {
        min-width: 100px;
    }
    .disc {
        min-width: 100px;
    }
}
</style>
