<script>
import axios from 'axios';
import EditField from '../components/edit-field.vue';

export default {
    props: [ 'item' ],
    data: function(){
        return {
            'editing': false,
            'full': '',
            'i' : {
              id: this.item.id,
              artist: this.item.artist,
              disc: this.item.disc,
            },
        };
    },
    components: {
        EditField,
    },
    watch: {
        full: function(n){
            const fields = n.split(' - ');
            this.i.artist = fields[0].trim();
            this.i.disc = fields[1].trim();
        },
    },
    methods: {
        save: function() {
            if (this.i.id) {
                axios.put('api/music/' + this.i.id, this.i).then(() => {
                    this.$emit('music-saved',this.i);
                });
            } else {
                axios.post('api/music', this.i).then(() => {
                    this.$emit('music-saved',this.i);
                });
            }
        },
        remove: function() {
            if (confirm("Sure you want to delete'")) {
              axios.delete('api/music/' + this.i.id).then(() => {
                  this.$emit('music-deleted',this.i);
              });
            }
        },
    }
}
</script>

<template>
    <div class="music-edit-form modal" >
        <input v-if="!i.id" class="music-helper" v-model="full"/>
        <edit-field label="artist" v-model="i.artist" />
        <edit-field label="disc"   v-model="i.disc" />
        <div class="modal-footer">
            <button class="close" v-on:click="$emit('close')">
              Close
            </button>
            <button class="save" v-on:click="save()">Save</button>
            <button class="remove" v-on:click="remove()" v-if="i.id">Delete</button>
        </div>
    </div>
</template>
