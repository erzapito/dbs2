<script>
export default {
    props: ['label','field'],
}
</script>

<template>
    <div class="edit-field">
        <label>{{ label }}</label>
        <input v-model="field" />
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
