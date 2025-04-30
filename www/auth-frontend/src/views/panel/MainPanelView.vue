<script setup lang="ts">

import api from '@/apis/auth_api.ts'
import { useRouter } from 'vue-router'

const router = useRouter();
router.beforeEach( async (to, from, next) => {
  if (to.path.includes('panel')) {
    await api.verify().then(() => {
      next();
    }).catch(() => {
      next(false);
      console.log("Not authenticated");
    })
  }else{
    next();
  }
})

</script>

<template>
  <main>
    <div class="navbar bg-base-100 shadow-sm p-3">
      <div class="navbar-start">
        <div class="dropdown">
          <div tabindex="0" role="button" class="btn btn-ghost btn-circle">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"> <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h7" /> </svg>
          </div>
          <ul
            tabindex="0"
            class="menu menu-sm dropdown-content bg-base-100 rounded-box z-1 mt-3 w-52 p-2 shadow">
            <li><RouterLink to="/panel/apps">Apps</RouterLink></li>
            <li><RouterLink to="/panel/users">Users</RouterLink></li>
            <li><RouterLink to="/logout">Log out</RouterLink></li>
          </ul>
        </div>
        <RouterLink to="/panel" class="btn btn-ghost text-xl">JoltAMP</RouterLink>
      </div>
    </div>
    <div class="view m-3">
      <RouterView />
    </div>
  </main>
</template>

<style scoped>

</style>
