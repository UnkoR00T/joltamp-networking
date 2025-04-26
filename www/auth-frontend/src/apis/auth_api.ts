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
            axios.post('/auth?app=Networking', null, {
                headers: {
                    Authorization: `${token}`
                }
            }).then((res) => {
                if(res.status == 401){
                    localStorage.removeItem('jwt');
                    window.location.href = '/site?app=Networking';
                    reject();
                }else{
                    resolve();
                }
            })
        })
    }
}

export default api;
