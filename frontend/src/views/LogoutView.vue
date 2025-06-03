<script setup lang="ts">
import { ref } from 'vue'

const byeMessage = ref("Wait...");
const addMessage = ref("We are loging you out!");

const onload = () => {
  setTimeout(()=> {
    localStorage.removeItem('jwt');
    byeMessage.value = 'Bye!';
    addMessage.value = 'Okey, you can go now, see you!';
  }, 2000)
}
onload();
</script>
<template>
  <main>
    <div class="hero bg-base-200 w-[25dvw] p-5 rounded-xl">
      <div class="hero-content text-center">
        <div class="max-w-md">
          <h1 class="text-5xl font-bold">{{byeMessage}}</h1>
          <p class="pt-3 text-md">{{addMessage}}</p>
        </div>
      </div>
    </div>
  </main>
</template>

<style scoped>
main{
  height: 100dvh;
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
}
</style>

