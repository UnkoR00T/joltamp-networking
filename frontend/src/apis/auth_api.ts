import axios from 'axios'

const api = {
    verify: (): Promise<void> => {
        return new Promise((resolve, reject) => {
            const token = localStorage.getItem('jwt');
            if (!token){
                localStorage.removeItem('jwt');
                window.location.href = '/?app=Networking';
                reject();
            }
            axios.post(`${import.meta.env.VITE_API_URL}/account/auth?app=Networking`, null, {
                headers: {
                    Authorization: `${token}`
                }
            }).then((res) => {
                if(res.status == 401 || res.status == 400){
                    localStorage.removeItem('jwt');
                    window.location.href = '/?app=Networking';
                    reject();
                }else{
                  if(!res.data){
                    alert("Cannot access this site!");
                    reject();
                  }
                    resolve();
                }
            }).catch(() => {
              localStorage.removeItem('jwt');
              window.location.href = '/?app=Networking';
              reject();
            })
        })
    }
}

export default api;
