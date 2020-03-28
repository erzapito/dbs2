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
