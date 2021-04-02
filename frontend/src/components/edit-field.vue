<script>
export default {
    props: ['label','value'],
}
</script>

<template>
    <div class="edit-field">
        <label>{{ label }}</label>
        <input
            v-bind:value="value"
            v-on:input="$emit('input', $event.target.value)" />
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
