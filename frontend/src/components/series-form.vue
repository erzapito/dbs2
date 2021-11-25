<script>
import axios from 'axios';
import EditField from '../components/edit-field.vue';

export default {
    props: [ 'item' ],
    emits: [ 'close', 'series-saved', 'series-deleted'],
    data: function(){
        return {
            'editing': false,
        };
    },
    components: {
        EditField,
    },
    computed: {
        saveIsDisabled: function() {
            return !this.item.name;
        },
    },
    methods: {
        save: function() {
            if (this.item.id) {
                axios.put('api/series/' + this.item.id, this.item).then(() => {
                    this.$emit('series-saved',this.item);
                });
            } else {
                axios.post('api/series', this.item).then(() => {
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
    <div class="series-edit modal">
        <edit-field label="name"     v-model="item.name" />
        <edit-field label="chapters" v-model="item.capitulos" />
        <edit-field label="category" v-model="item.categoria" />
        <edit-field label="fansub"   v-model="item.fansub" />
        <edit-field label="idioma"   v-model="item.idioma" />
        <div class="modal-footer">
            <button class="close" v-on:click="$emit('close')">
              Close
            </button>
            <button class="save" v-on:click="save()" :disabled="saveIsDisabled">
              Save
            </button>
            <button class="remove" v-on:click="remove()" v-if="item.id">
              Delete
            </button>
        </div>
    </div>
</template>

<style lang="scss">
@import '../assets/modal.scss';
</style>
