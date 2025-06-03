<script setup lang="ts">

import { useRoute, useRouter } from 'vue-router'
import { onBeforeMount, type Ref, ref } from 'vue'
import axios from 'axios'
import api from '@/apis/auth_api.ts'

const route = useRoute();
const router = useRouter();
const app = route.params.id;
const token = localStorage.getItem('jwt');
const perms_modal = ref(false);
const data: Ref<{name: string, perms: string[], url: string}> = ref({name: '', perms: [], url: ''});
const permsSaved = ref(true);
const fetchData = async () => {
  if(token) {
    axios.get(`${import.meta.env.VITE_API_URL}/app?app=${app}`, {
      headers: {
        Authorization: `${token}`
      }
    }).then((res)=>{
      data.value = res.data;
    })
  }
}
onBeforeMount(() => {
  fetchData();
})
const remove = (app: string) => {
  if (confirm("Are you sure you want to permanently delete this app?")){
    api.verify().then(() => {
      axios.post(`${import.meta.env.VITE_API_URL}/panel/app/remove`, {
          app_id: app
        }, {
          headers: {
            Authorization: `${token}`,
          }
        }).then((res) => {
          if(res.status === 200){
            router.push("/panel/apps");
          }
        }).catch((err) => {
          if(err.status === 406){
            alert("This app cannot be changed!")
          }
        })
    }).catch(() => {})
  }
}
const savePerms = () => {
  if (confirm("Are you sure you want to save?")){
    api.verify().then(() => {
      axios.post(`${import.meta.env.VITE_API_URL}/panel/app/change_perms`, {
        app_id: app,
        perms: data.value.perms
      }, {
        headers: {
          Authorization: `${token}`,
        }
      }).then((res) => {
        if(res.status === 200){
          openPermsModal();
          permsSaved.value = true;
        }
      }).catch((err) => {
        if(err.status === 406){
          alert("This app cannot be changed!");
        }
      })
    })
  }
}
const changeAppId = () => {
  api.verify().then(() => {}).catch(() => {return;})
  const appId = prompt("Enter new app id:");
  if (confirm("Are you sure you want to save?")){
    if(token && appId){
      axios.post(`${import.meta.env.VITE_API_URL}/panel/app/change_id`, {
        cur_app_id: app,
        new_app_id: appId
      }, {
        headers: {
          Authorization: `${token}`,
        }
      }).then((res) => {
        if(res.status === 200){
          data.value.name = appId;
          router.push("/panel/app/" + appId);
        }
      }).catch((err) => {
        if(err.status === 406){
          alert("This app cannot be changed!");
        }
      })
    }
  }
}
const changeAppUrl = () => {
  api.verify().then(() => {}).catch(() => {return;})
  const appUrl = prompt("Enter new app url:");
  if (confirm("Are you sure you want to save?")){
    if(token && appUrl){
      axios.post(`${import.meta.env.VITE_API_URL}/panel/app/change_url`, {
        app_id: app,
        url: appUrl
      }, {
        headers: {
          Authorization: `${token}`,
        }
      }).then((res) => {
        if(res.status === 200){
          data.value.url = appUrl;
        }
      }).catch((err) => {
        if(err.status === 406){
          alert("This app cannot be changed!");
        }
      })
    }
  }
}
const openPermsModal = () => {
  perms_modal.value = !perms_modal.value;
}
const addPerms = () => {
  permsSaved.value = false;
  data.value.perms.push('');
}
const removePerm = (item: string) => {
  console.log(item)
  data.value.perms = data.value.perms.filter(x => x !== item);
}
</script>

<template>
  <main class="mx-5">
    <div class="absolute h-[70%] w-[calc(100%-34px)]" v-if="perms_modal">
      <div class="grid bg-base-200 p-5 w-[500px] relative rounded-xl" style="top: 50%; left: 50%; transform: translate(-50%, -50%);">
        <h3 class="text-lg font-bold">App Perms</h3>
        <div class="my-3 flex flex-col gap-3">
          <div class="join w-[100%]" v-for="(item, index) in data.perms" :key="index">
            <input type="text" class="input join-item w-[80%]" v-model="data.perms[index]" placeholder="New perm! DO NOT LEFT EMPTY">
            <button class="btn btn-error join-item w-[20%]" @click="removePerm(item)">Del</button>
          </div>
        </div>
        <div class='flex justify-between'>
          <button class="btn btn-primary" @click="addPerms()">Add</button>
          <div class="flex gap-1">
            <button class="btn btn-primary" @click="savePerms()">Save</button>
            <button class="btn btn-error" @click="openPermsModal()">Close</button>
          </div>
        </div>
      </div>
    </div>
    <div>
      <div class="flex md:flex-row flex-col gap-10">
        <div>
          <h1 class="text-3xl mb-3">{{data.name}}</h1>
          <p>
            App Id: <span class="text-primary">{{data.name}}</span>
          </p>
          <p>
            Name: <span class="text-primary">{{data.name}}</span>
          </p>
          <p>
            Perms: <span class="text-primary">{{data.perms}}</span> <span v-if="!permsSaved" class="text-error">UNSAVED!</span>
          </p>
          <p>
            Url: <a :href="data.url" class="link-primary">{{data.url}}</a>
          </p>
        </div>
        <div>
          <h1 class="text-3xl mb-3">Edit</h1>
          <p class="link-primary cursor-pointer" @click="changeAppId()">
            Edit app id
          </p>
          <p class="link-primary cursor-pointer" @click="openPermsModal()">
            Edit perms
          </p>
          <p class="link-primary cursor-pointer" @click="changeAppUrl()">
            Edit app url
          </p>
          <p class="link-error cursor-pointer" @click="remove(data.name)">
            Delete app
          </p>
        </div>
      </div>
    </div>
  </main>
</template>

<style scoped>

</style>
