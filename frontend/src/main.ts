import { createApp } from 'vue'
import { createPinia } from 'pinia'

import './style.css'

import App from './App.vue'
import router from './router'
import {setupPlausible} from "@/plugins/plausible.ts";

const app = createApp(App)

app.use(createPinia())
app.use(router)

setupPlausible(app);

app.mount('#app')
