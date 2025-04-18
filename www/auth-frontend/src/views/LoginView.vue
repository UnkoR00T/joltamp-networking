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
    fetch("/account/login", {
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
            router.push("/?app=" + authorizeStore.appId);
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
    <div class="login-card">
      <h1>Welcome Back</h1>
      <p>Please sign in to your account</p>
      <form @submit.prevent="submitForm">
        <div class="form-group">
          <label for="email">Email</label>
          <input
            v-model="form.email"
            type="email"
            id="email"
            placeholder="you@example.com"
            required
          />
        </div>

        <div class="form-group">
          <label for="password">Password</label>
          <input
            v-model="form.password"
            type="password"
            id="password"
            placeholder="••••••••"
            required
          />
        </div>

        <div class="form-options">
          <label class="remember">
            <input v-model="form.remember" type="checkbox" />
            Remember me
          </label>
          <a href="#" class="forgot">Forgot password?</a>
        </div>
        <span class="error" v-if="error">{{error}}</span>
        <button type="submit">Login</button>
      </form>

      <p class="signup">
        Don’t have an account?
        <RouterLink to="/register" class="a">Sign up</RouterLink>
      </p>
    </div>
  </main>
</template>

<style scoped>
.error{
  color: #ed0000;
}
main {
  height: 100dvh;
  display: flex;
  justify-content: center;
  align-items: center;
  background: #f0f4f8;
}

.login-card {
  background: #ffffff;
  padding: 2rem;
  border-radius: 12px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
  width: 100%;
  max-width: 360px;
  text-align: center;
}

.login-card h1 {
  margin: 0 0 0.5rem;
  font-size: 1.75rem;
}

.login-card p {
  margin: 0 0 1.5rem;
  color: #555;
}

.form-group {
  margin-bottom: 1.25rem;
  text-align: left;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
}

.form-group input {
  box-sizing: border-box;
  width: 100%;
  padding: 0.75rem 1rem;
  font-size: 1rem;
  border: 1px solid #ccd0d5;
  border-radius: 6px;
  transition: border-color 0.2s;
}

.form-group input:focus {
  outline: none;
  border-color: #4a90e2;
}

.form-options {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
  font-size: 0.875rem;
}

.form-options .remember {
  display: flex;
  align-items: center;
  gap: 0.25rem;
}

.form-options .forgot {
  color: #4a90e2;
  text-decoration: none;
}

.form-options .forgot:hover {
  text-decoration: underline;
}

button {
  width: 100%;
  padding: 0.75rem 0;
  font-size: 1rem;
  font-weight: 600;
  color: #fff;
  background: #4a90e2;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.2s;
}

button:hover {
  background: #357abd;
}

.signup {
  margin-top: 1.25rem;
  font-size: 0.875rem;
  color: #555;
}

.signup > .a {
  color: #4a90e2;
  text-decoration: none;
  font-weight: 500;
}

.signup a:hover {
  text-decoration: underline;
}
</style>
