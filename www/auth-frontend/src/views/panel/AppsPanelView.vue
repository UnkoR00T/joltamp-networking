<script setup lang="ts">
import { onBeforeMount, type Ref, ref } from 'vue'
import axios from 'axios'
import { useRouter } from 'vue-router'
import api from '@/apis/auth_api.ts'

const data: Ref<
  { id: { tb: string; id: { String: string } }; name: string; perms: string[]; url: string }[]
> = ref([])
const page = ref(1)
const limit = ref(15)
const router = useRouter();
const perms_modal = ref(false);
const newApp: Ref<{name: string, perms: string[], url: string}> = ref({name: '', perms: [], url: ''})

const token = localStorage.getItem('jwt')
const fetchData = async (): Promise<void> => {
  return new Promise<void>((resolve, reject) => {
    axios
      .get(`${import.meta.env.VITE_API_URL}/panel/apps?page=${page.value}&limit=${limit.value}`, {
        headers: {
          Authorization: `${token}`,
        },
      })
      .then((res) => {
        data.value = res.data
        resolve();
      }).catch((err) => {reject(err)})
  })
}
const info = (app: string) => {
  router.push("/panel/app/" + app);
}
const remove = (app: string) => {
  if (confirm("Are you sure you want to permanently delete this app?")){
    if(token){
      axios.post(`${import.meta.env.VITE_API_URL}/panel/app/remove`, {
        app_id: app
      }, {
        headers: {
          Authorization: `${token}`,
        }
      }).then((res) => {
        if(res.status === 200){
          data.value = data.value.filter(x => x.name !== app);
        }
      }).catch(() => {})
    }
  }
}
onBeforeMount(() => {
  fetchData()
})
const addPerms = () => {
  newApp.value.perms.push('');
}
const removePerm = (item: string) => {
  console.log(item)
  newApp.value.perms = newApp.value.perms.filter(x => x !== item);
}
const openPermsModal = () => perms_modal.value = !perms_modal.value;
const saving = ref(false);
const saveApp = async () => {
  saving.value = true;
  if (confirm("Are you sure you want to save this app?")) {
    api.verify().then(() =>{
      axios.post(`${import.meta.env.VITE_API_URL}/app/create`, {
        name: newApp.value.name,
        perms: newApp.value.perms,
        url: newApp.value.url
      }, {
        headers: {
          Authorization: `${token}`
        }
      }).then(async () => {
        await fetchData();
        openPermsModal();
        saving.value = false;
      })
    })
  }
}
</script>

<template>
  <div class="overflow-x-auto ml-5 mr-5">
    <div class="absolute h-[70%] w-[calc(100%-34px)] z-9999" v-if="perms_modal">
      <div class="grid bg-base-200 p-5 w-[500px] relative rounded-xl" style="top: 50%; left: 50%; transform: translate(-50%, -50%);">
        <h3 class="text-lg font-bold">New App</h3>
        <div class="my-3 flex flex-col gap-3">
          <div class="flex flex-col gap-3">
            <input type="text" id="name" class="input w-[100%]" placeholder="Name/Id" v-model="newApp.name">
            <input type="text" id="name" class="input w-[100%]" placeholder="App auth URL" v-model="newApp.url">
          </div>
          <h4>Perms:</h4>
          <div class="flex flex-col gap-3 p-2 pt-0">
            <div class="join w-[100%]" v-for="(item, index) in newApp.perms" :key="index">
              <input type="text" class="input join-item w-[80%]" v-model="newApp.perms[index]" placeholder="New perm!">
              <button class="btn btn-error join-item w-[20%]" @click="removePerm(item)">Del</button>
            </div>
            <button class="btn btn-primary" @click="addPerms()">Add</button>

          </div>
        </div>
        <div class='flex justify-between'>
          <div class="flex gap-1">
            <button class="btn btn-primary" @click="saveApp()">
              <span v-if="!saving">Save</span>
              <span class="loading loading-dots loading-lg" v-else></span>
            </button>
            <button class="btn btn-error" @click="openPermsModal()">Close</button>
          </div>
        </div>
      </div>
    </div>
    <table class="table table-zebra">
      <!-- head -->
      <thead>
        <tr>
          <th>Id</th>
          <th>Perms</th>
          <th>URL</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        <!-- row 1 -->
        <tr v-for="record in data" :key="record.name">
          <th>{{ record.id.id.String }}</th>
          <td>{{ record.perms }}</td>
          <td><a :href="record.url" class="link-primary">Click/Hover</a></td>
          <td class="flex join">
            <button class="btn btn-primary join-item" @click="info(record.name)">Info</button>
            <button class="btn btn-error join-item" @click="remove(record.name)">Delete</button>
          </td>
        </tr>
      </tbody>
    </table>
    <div class="flex justify-center mt-5 join">
      <button class="btn btn-accent join-item px-5 w-[100px]" @click="openPermsModal">New</button>
      <input type="number" placeholder="Page" class="input w-50 join-item" v-model="page" />
      <input type="number" placeholder="Limit" class="input w-50 join-item" v-model="limit" />
      <button class="btn btn-primary join-item px-5 w-[100px]" @click="fetchData">Search</button>
    </div>
  </div>
</template>

<style scoped></style>
