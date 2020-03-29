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
    watch: {
        full: function(n){
            const fields = n.split(' - ');
            this.item.artist = fields[0].trim();
            this.item.disc = fields[1].trim();
        },
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
    }
}
</script>

<template>
    <div class="music-edit-form" >
        <input v-if="!item.id" class="music-helper" v-model="full"/>

        <edit-field label="artist" v-bind:field="item.artist" />
        <edit-field label="disc" v-bind:field="item.disc" />
        <div class="buttons">
            <button class="save" v-on:click="save()">Save</button>
            <button class="remove" v-on:click="remove()" v-if="item.id">Delete</button>
        </div>
    </div>
</template>
