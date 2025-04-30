<script setup lang="ts">
import { onBeforeMount, type Ref, ref } from 'vue'
import axios from 'axios'
import { useRouter } from 'vue-router'

const data: Ref<
  { id: { tb: string; id: { Uuid: string } }; firstname: string; lastname: string; email: string }[]
> = ref([])
const page = ref(1)
const limit = ref(15)
const router = useRouter();

const token = localStorage.getItem('jwt')
const fetchData = async () => {
  if (token) {
    axios
      .get(`${import.meta.env.VITE_API_URL}/panel/users?page=${page.value}&limit=${limit.value}`, {
        headers: {
          Authorization: `${token}`,
        },
      })
      .then((res) => {
        data.value = res.data
      })
  }
}
const remove = (user_id: string) => {
  if (confirm("Are you sure you want to permanently delete this user?")){
    if(token){
      axios.post(`${import.meta.env.VITE_API_URL}/panel/user/remove`, {
        user_id: user_id
      }, {
        headers: {
          Authorization: `${token}`,
        }
      }).then((res) => {
        if(res.status === 200){
          data.value = data.value.filter(x => x.id.id.Uuid !== user_id);
        }
      }).catch(() => {})
    }
  }
}
onBeforeMount(() => {
  fetchData();
})
</script>

<template>
  <div class="overflow-x-auto ml-5 mr-5">
    <table class="table table-zebra">
      <!-- head -->
      <thead>
      <tr>
        <th>Id</th>
        <th>Firstname</th>
        <th>Lastname</th>
        <th>Email</th>
        <th>Actions</th>
      </tr>
      </thead>
      <tbody>
      <!-- row 1 -->
      <tr v-for="record in data" :key="record.id.id.Uuid">
        <th>{{ record.id.id.Uuid }}</th>
        <td>{{ record.firstname }}</td>
        <td>{{ record.lastname }}</td>
        <td>{{ record.email }}</td>
        <td class="gap-2 flex">
          <button class="btn btn-error" @click="remove(record.id.id.Uuid)">Delete</button>
        </td>
      </tr>
      </tbody>
    </table>
    <div class="flex justify-center mt-5 join">
      <input type="number" placeholder="Page" class="input w-50 join-item" v-model="page" />
      <input type="number" placeholder="Limit" class="input w-50 join-item" v-model="limit" />
      <button class="btn btn-primary join-item px-5" @click="fetchData">Search</button>
    </div>
  </div>
</template>

<style scoped></style>
