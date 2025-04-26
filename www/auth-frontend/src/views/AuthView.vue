<script setup lang="ts">
import { useAuthorizeStore } from '@/stores/authorizeStore.ts'
import { useRouter } from 'vue-router'

const jwt = localStorage.getItem('jwt');
const authorizeStore = useAuthorizeStore();
const router = useRouter()
//const route = useRoute()

const permsLang: Record<'email' | 'firstname' | 'lastname' | 'other', string> = {
  email: "Your email address",
  firstname: "Your First Name",
  lastname: "Your Last Name",
  other: "Your other authorized apps"
}
type PermKey = 'email' | 'firstname' | 'lastname' | 'other';

const perms: PermKey[] = ['email', 'firstname', 'lastname', 'other'];

if(!authorizeStore.appUrl){
  router.push('/404')
}

fetch(`${import.meta.env.VITE_API_URL}/app/auth?app=${authorizeStore.appId}`, {
  method: 'POST',
  headers: {
    'Authorization': `${jwt}`,
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    authorize: false
  })
})
  .then((response) => {
    if(response.ok){
      if(response.status !== 202){
        router.push('/')
      }
    }
  });

const submit = () => {
  fetch(`${import.meta.env.VITE_API_URL}/app/auth?app=${authorizeStore.appId}`, {
    method: 'POST',
    headers: {
      'Authorization': `${jwt}`,
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({
      authorize: true
    })
  })
  .then((response) => {
    response.json()
    if(response.ok){
      window.location.href = authorizeStore.appUrl + "?token=" + jwt;
    }
  });
}

</script>
<template>
  <main>
    <form @submit.prevent="submit" class="p-5 bg-base-200 rounded-xl w-[400px]">
      <h1 class="text-3xl font-bold text-center">Authorize App</h1>
      <p class="text-center mb-5"><span class="text-primary">{{ authorizeStore.appId }}</span>, this app will be able to access:</p>
      <div class="pb-5">
        <div v-if="authorizeStore.appPerms[0] !== '*'" class="grid gap-2">
          <div
            class="card rounded-box grid p-2 text-center"

            v-for="item in authorizeStore.appPerms as PermKey[]"
            v-bind:key="item"
          >
            {{ permsLang[item] }}
          </div>
        </div>
        <div v-else class="grid gap-2">
          <div
            class="card rounded-box grid p-2 text-center"
            v-for="item in perms"
            v-bind:key="item"
          >
            {{ permsLang[item] }}
          </div>
        </div>
      </div>

      <button class="btn btn-primary w-[100%] text-base-200" type="submit">Authorize</button>
    </form>
  </main>
</template>

<style scoped>
main {
  height: 100dvh;
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>

