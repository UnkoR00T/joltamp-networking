<script setup lang="ts">
import { reactive, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthorizeStore } from '@/stores/authorizeStore.ts'

interface LoginForm {
  firstname: string;
  lastname: string;
  email: string;
  password: string;
  remember: boolean;
}

const form = reactive<LoginForm>({
  firstname: '',
  lastname: '',
  email: '',
  password: '',
  remember: false,
});

const router =useRouter();
const error = ref("");
const authorizeStore = useAuthorizeStore();

function submitForm() {
  if(form.email.length > 3 && form.password.length > 3 && form.firstname.length > 3 && form.password.length > 3){
    fetch(`${import.meta.env.VITE_API_URL}/account/register`, {
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        firstname: form.firstname,
        lastname: form.lastname,
        email: form.email,
        password: form.password
      }),
      method: 'POST'
    }).then(async res => {
      error.value = "";
      if(res.ok){
        const data = await res.json();
        localStorage.setItem("jwt", data.jwt);
        if(authorizeStore.appId){
          router.push("/?app=" + authorizeStore.appId);
        }
      }else{
        error.value = "Wrong email or account already exists."
      }
    })
  }else{
    error.value = "Please fill all fields."
  }
}
</script>

<template>
  <main>
    <div class="p-5 bg-base-200 rounded-xl w-[400px]">
      <h1 class="text-3xl font-bold text-center">Welcome</h1>
      <p class="text-center mb-5">Please sign up your account</p>
      <form @submit.prevent="submitForm">
        <div>
          <label class="input w-[100%]">
            <svg class="h-[1em] opacity-50" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
              <g
                stroke-linejoin="round"
                stroke-linecap="round"
                stroke-width="2.5"
                fill="none"
                stroke="currentColor"
              >
                <path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"></path>
                <circle cx="12" cy="7" r="4"></circle>
              </g>
            </svg>
            <input
              v-model="form.firstname"
              type="text"
              required
              pattern="[A-Za-z][A-Za-z0-9\-]*"
              placeholder="First name"
              minlength="3"
              maxlength="30"
            />
          </label>
        </div>

        <div class="mt-5 mb-2">
          <label class="input w-[100%]">
            <svg class="h-[1em] opacity-50" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
              <g
                stroke-linejoin="round"
                stroke-linecap="round"
                stroke-width="2.5"
                fill="none"
                stroke="currentColor"
              >
                <path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"></path>
                <circle cx="12" cy="7" r="4"></circle>
              </g>
            </svg>
            <input
              v-model="form.lastname"
              type="text"
              required
              placeholder="Last name"
              minlength="3"
              maxlength="30"
            />
          </label>
        </div>

        <div class="mt-5 mb-2">
          <label class="input w-[100%]">
            <svg class="h-[1em] opacity-50" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
              <g
                stroke-linejoin="round"
                stroke-linecap="round"
                stroke-width="2.5"
                fill="none"
                stroke="currentColor"
              >
                <rect width="20" height="16" x="2" y="4" rx="2"></rect>
                <path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"></path>
              </g>
            </svg>
            <input
              v-model="form.email"
              type="email"
              required
              placeholder="Email"
              minlength="3"
              maxlength="30"
              class="w-[100%]"
            />
          </label>
        </div>

        <div class="mt-5 mb-2">
          <label class="input w-[100%]">
            <svg class="h-[1em] opacity-50" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
              <g
                stroke-linejoin="round"
                stroke-linecap="round"
                stroke-width="2.5"
                fill="none"
                stroke="currentColor"
              >
                <path
                  d="M2.586 17.414A2 2 0 0 0 2 18.828V21a1 1 0 0 0 1 1h3a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h1a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h.172a2 2 0 0 0 1.414-.586l.814-.814a6.5 6.5 0 1 0-4-4z"
                ></path>
                <circle cx="16.5" cy="7.5" r=".5" fill="currentColor"></circle>
              </g>
            </svg>
            <input
              v-model="form.password"
              type="password"
              required
              placeholder="Password"
              minlength="3"
              maxlength="30"
            />
          </label>
        </div>

        <div class="flex justify-between mb-7">
          <label class="">
            <input type="checkbox" class="toggle toggle-primary toggle-sm" v-model="form.remember"/>
            Remember me
          </label>
        </div>
        <span class="text-error text-sm" v-if="error">{{error}}</span>
        <button class="btn btn-primary w-[100%] text-base-200" type="submit">Register</button>
      </form>

      <p class="signup text-sm">
        You have an account?
        <RouterLink to="/login" class="text-primary hover:underline">Sign in</RouterLink>
      </p>
      <p class="text-xs mt-5 text-center">&#169; Copyright by JoltAMP.pl</p>
    </div>
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
