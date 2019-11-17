<script>
import axios from 'axios';

import EditField from '../components/edit-field.vue';

export default {
    props: [ 'item' ],
    data: function(){
        return {
            'editing': false,
        };
    },
    components: {
        EditField,
    },
    methods: {
        save: function() {
            axios.put('/api/series/' + this.item.id, {
                    params : this.item,
            }).then((response) => {
                console.log("item saved: " + response);
            });
        },
        remove: function() {
            axios.delete('/api/series/' + this.item.id).then((response) => {
                console.log("item deleted: " + response);
            });
        }
    }
}
</script>

<template>
    <div class="series-item">
        {{ item.id }}
        <a href="javascript:void(0)" v-on:click="editing = !editing">{{ item.name }}</a>
        <div v-if="editing" class="edit-form" >
            <edit-field label="name" v-bind:field="item.name" />
            <edit-field label="chapters" v-bind:field="item.capitulos" />
            <edit-field label="category" v-bind:field="item.category" />
            <edit-field label="fansub" v-bind:field="item.fansub" />
            <edit-field label="idioma" v-bind:field="item.idioma" />
            <button v-on:click="save()">Save</button>
            <button v-on:click="remove()">Delete</button>
        </div>
    </div>
</template>

<style>
.series-item {
    display: inline-block;
}

</style>
