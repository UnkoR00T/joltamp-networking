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

fetch("http://localhost:8080/app/auth?app=" + authorizeStore.appId, {
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
  fetch("http://localhost:8080/app/auth?app=" + authorizeStore.appId, {
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
    <form @submit.prevent="submit" class="auth-card">
      <h2>Authorize App</h2>
      <p class="app-name">{{ authorizeStore.appId }}</p>
      <p class="subtitle">This app will be able to access:</p>

      <div class="perm-list">
        <div
          class="perm-item"
          v-for="item in authorizeStore.appPerms as PermKey[]"
          v-if="authorizeStore.appPerms[0] !== '*'"
        >
          {{ permsLang[item] }}
        </div>

        <div
          class="perm-item"
          v-for="item in perms"
          v-else
        >
          {{ permsLang[item] }}
        </div>
      </div>

      <button type="submit">Authorize</button>
    </form>
  </main>
</template>

<style scoped>
main {
  height: 100dvh;
  display: flex;
  justify-content: center;
  align-items: center;
  background: #f0f4f8;
}

.auth-card {
  background: #ffffff;
  padding: 2rem;
  border-radius: 12px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
  width: 100%;
  max-width: 420px;
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
}

h2 {
  margin: 0 0 0.5rem;
  font-size: 1.75rem;
}

.app-name {
  font-size: 1.25rem;
  font-weight: 600;
  color: #333;
  margin: 0.25rem 0 1.5rem;
}

.subtitle {
  font-size: 0.95rem;
  color: #666;
  margin-bottom: 1rem;
}

.perm-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  width: 100%;
}

.perm-item {
  padding: 0.75rem 1rem;
  background: #f9fafb;
  border: 1px solid #e0e3e7;
  border-radius: 8px;
  text-align: left;
  box-shadow: 0 2px 6px rgba(0,0,0,0.05);
}

button {
  margin-top: 1.75rem;
  width: 100%;
  padding: 0.75rem 0;
  font-size: 1rem;
  font-weight: 600;
  color: #fff;
  background: #4a90e2;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: background 0.2s ease;
}

button:hover {
  background: #357abd;
}
</style>

