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
            if (this.item.id) {
                axios.put('api/series/' + this.item.id, this.item).then(() => {
                    this.$emit('series-saved',this.item);
                });
            } else {
                axios.post('api/series/', this.item).then(() => {
                    this.$emit('series-saved',this.item);
                });
            }
        },
        remove: function() {
            axios.delete('api/series/' + this.item.id).then(() => {
                this.$emit('series-deleted',this.item);
            });
        }
    }
}
</script>

<template>
    <div class="series-edit" >
        <edit-field label="name" v-bind:field="item.name" />
        <edit-field label="chapters" v-bind:field="item.capitulos" />
        <edit-field label="category" v-bind:field="item.categoria" />
        <edit-field label="fansub" v-bind:field="item.fansub" />
        <edit-field label="idioma" v-bind:field="item.idioma" />
        <div class="buttons">
            <button class="save" v-on:click="save()">Save</button>
            <button class="remove" v-on:click="remove()" v-if="item.id">Delete</button>
        </div>
    </div>
</template>
