<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router'

const router = useRouter();
const route = useRoute();
const token = route.query.token;
const onload = async () => {
  if(!token){
    window.location.href = '/?app=Networking';
  }else{
    router.push('/panel');
  }
}
onload();
</script>
<template>
  <main></main>
</template>

<style scoped>

</style>
