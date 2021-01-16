<script>
export default {
    props: ['label','field'],
    data: function() {
      return {
        value: '',
      };
    },
    mounted: function(){
      this.value = this.field;
    },
    watch: {
        field: function(n){
          this.value = n;
        },
    },
}
</script>

<template>
    <div class="edit-field">
        <label>{{ label }}</label>
        <input v-model="value" v-on:keyup="$emit('value-changed',value)" />
    </div>
</template>

<style lang="scss">
.edit-field {
    display: flex;

    label {
        display: flex;
        width: 100px;
        align-items: center;
        justify-content: left;
    }
}
</style>
