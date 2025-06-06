import axios from 'axios'
import { useRouter } from 'vue-router'

const router = useRouter();

const api = {
    verify: (): Promise<void> => {
        return new Promise((resolve, reject) => {
            const token = localStorage.getItem('jwt');
            if (!token){
                localStorage.removeItem('jwt');
                window.location.href = '/site?app=Networking';
                reject();
            }
            axios.post(`${import.meta.env.VITE_API_URL}/account/auth?app=Networking`, null, {
                headers: {
                    Authorization: `${token}`
                }
            }).then((res) => {
                if(res.status == 401 || res.status == 400){
                    localStorage.removeItem('jwt');
                    window.location.href = '/site?app=Networking';
                    reject();
                }else{
                  if(!res.data){
                    alert("Cannot access this site!");
                    reject();
                  }
                    resolve();
                }
            }).catch((err) => {
              localStorage.removeItem('jwt');
              window.location.href = '/site?app=Networking';
              reject();
            })
        })
    }
}

export default api;
