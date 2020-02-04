<script>
import axios from 'axios';
import EditField from '../components/edit-field.vue';

export default {
    props: [ 'item' ],
    data: function(){
        return {
            'editing': false,
            'full': '',
        };
    },
    components: {
        EditField,
    },
    methods: {
        save: function() {
            if (this.item.id) {
                axios.put('/api/music/' + this.item.id, this.item).then(() => {
                    this.$emit('music-saved',this.item);
                });
            } else {
                axios.post('/api/music/', this.item).then(() => {
                    this.$emit('music-saved',this.item);
                });
            }
        },
        remove: function() {
            axios.delete('/api/music/' + this.item.id).then(() => {
                this.$emit('music-deleted',this.item);
            });
        },
        fullChanged: function() {
            const fields = this.full.split(' - ');
            this.item.artist = fields[0];
            this.item.disc = fields[1];
        },
    }
}
</script>

<template>
    <div class="edit-form" >
        <input class="music-helper" v-model="full" v-on:change="fullChanged()"/>

        <edit-field label="artist" v-bind:field="item.artist" />
        <edit-field label="disc" v-bind:field="item.disc" />
        <button class="save" v-on:click="save()">Save</button>
        <button class="remove" v-on:click="remove()" v-if="item.id">Delete</button>
    </div>
</template>
