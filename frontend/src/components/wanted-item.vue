<script>
export default {
    props: [ 'item' ],
    methods: {}
}
</script>

<template>
    <tr class="wanted-item">
        <td class="id">{{ item.id }}</td>
        <td class="artist">{{ item.artist }}</td>
        <td class="disc">{{ item.disc }}</td>
        <td class="weeks">{{ item.weeks }}</td>
        <td class="done">{{ item.done }}</td>
    </tr>
</template>

<style lang="scss">
.series-item {
    display: inline-block;
}

</style>
