<script setup lang="ts">

import { useRoute } from 'vue-router'
import { onBeforeMount } from 'vue'

const route = useRoute();
const app = route.params.id;
const fetchData = async () => {

}
onBeforeMount(() => {
  fetchData();
})

</script>

<template>

</template>

<style scoped>

</style>
