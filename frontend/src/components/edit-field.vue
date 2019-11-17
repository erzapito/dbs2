<script>
export default {
    props: ['label','field'],
}
</script>

<template>
    <div>
        <label>{{ label }}</label>
        <input v-model="field" />
    </div>
</template>