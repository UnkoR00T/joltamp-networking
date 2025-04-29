<script setup lang="ts">
import { type Ref, ref } from 'vue'
import axios from 'axios'
import { useRouter } from 'vue-router'

const data: Ref<
  { id: { tb: string; id: { String: string } }; name: string; perms: string[]; url: string }[]
> = ref([])
const page = ref(1)
const limit = ref(15)
const router = useRouter();

const token = localStorage.getItem('jwt')
if (token) {
  axios
    .get(`${import.meta.env.VITE_API_URL}/panel/apps?page=${page.value}&limit=${limit.value}`, {
      headers: {
        Authorization: `${token}`,
      },
    })
    .then((res) => {
      data.value = res.data
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
</script>

<template>
  <div class="overflow-x-auto ml-5 mr-5">
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
          <td class="gap-2 flex">
            <button class="btn btn-primary" @click="info(record.name)">Info</button>
            <button class="btn btn-error" @click="remove(record.name)">Delete</button>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<style scoped></style>
