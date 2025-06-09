<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router'
import { useAuthorizeStore } from '@/stores/authorizeStore.ts'
const router = useRouter()
const route = useRoute()
const app = route.query.app
let appUrl = "";
const jwt = localStorage.getItem('jwt');
const authorizeStore = useAuthorizeStore();
if(app){
  authorizeStore.setAppId(app.toString());
}
console.log(jwt);
if(app){
  authorizeStore.setAppId(app.toString());
}
if(jwt){
  fetch(`${import.meta.env.VITE_API_URL}/account/verify`, {
    method: 'POST',
    headers: {
      Authorization: `${jwt}`
    }
  }).then(res => {
    if(res.ok){
      res.json();
      appCheck();
    }else{
      localStorage.removeItem('jwt');
      router.push('/login');
    }
  })
}else{
  router.push('/login');
}

const appCheck = async () => {
  fetch(`${import.meta.env.VITE_API_URL}/app?app=${app}`, {
    method: 'GET',
    headers: {
      Authorization: `${jwt}`,
    }
  }).then(async (res) => {
    if(res.ok){
      const data = await res.json();
      appUrl = data.url;
      const appPerms = data.perms;
      authorizeStore.setAppUrl(appUrl.toString());
      authorizeStore.setAppPerms(appPerms);
      authCheck();
    }else{
      if(res.status === 401){
        localStorage.removeItem('jwt');
        if(app){
          authorizeStore.setAppId(app.toString());
        }
        router.push('/login');
      }else if(res.status === 404){
        router.push('/404');
      }
    }
  });
}

const authCheck = async () => {
  fetch(`${import.meta.env.VITE_API_URL}/account/auth?app=${app}`, {
    method: 'POST',
    headers: {
      Authorization: `${jwt}`,
    }
  }).then(async (res) => {
    if(res.ok){
      window.location.href = appUrl + "?token=" + jwt;
    }else{
      router.push('/auth')
    }
  }).catch(()=>{
    router.push('/auth')
  })
}

</script>

<template>
  <main>
    <span class="loading loading-spinner loading-xl"></span>
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
