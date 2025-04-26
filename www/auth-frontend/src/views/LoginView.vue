<script setup lang="ts">
import { reactive, ref } from 'vue'
import { useAuthorizeStore } from '@/stores/authorizeStore.ts'
import { useRouter } from 'vue-router'

interface LoginForm {
  email: string;
  password: string;
  remember: boolean;
}

const form = reactive<LoginForm>({
  email: '',
  password: '',
  remember: false,
});

const router =useRouter();
const error = ref("");
const authorizeStore = useAuthorizeStore();

function submitForm() {
  if(form.email.length > 3 && form.password.length > 3){
    fetch(`${import.meta.env.VITE_API_URL}/account/login`, {
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
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
            if(authorizeStore.appId === "app_self"){
              await router.push("/panel");
            }else{
              await router.push("/?app=" + authorizeStore.appId);
            }
          }
        }else{
          error.value = "Password/Email is incorrect, or account dont exists."
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
      <h1 class="text-3xl font-bold text-center">Welcome Back</h1>
      <p class="text-center mb-5">Please sign in to your account</p>
      <form @submit.prevent="submitForm">
        <div class="w-[100%]">
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
              title="Only letters, numbers or dash"
            />
          </label>
        </div>

        <div class="flex justify-between mb-7">
          <label class="">
            <input type="checkbox" class="toggle toggle-primary toggle-sm" v-model="form.remember"/>
            Remember me
          </label>
          <a href="#" class="text-primary hover:underline">Forgot password?</a>
        </div>
        <span class="text-error text-sm" v-if="error">{{error}}</span>
        <button class="btn btn-primary w-[100%] text-base-200" type="submit">Login</button>
      </form>

      <p class="signup text-sm">
        Don’t have an account?
        <RouterLink to="/register" class="text-primary hover:underline">Sign up</RouterLink>
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
