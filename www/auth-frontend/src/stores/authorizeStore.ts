import { defineStore } from 'pinia'

export const useAuthorizeStore = defineStore('authorizeStore', {
  // arrow function recommended for full type inference
  state: (): {
    appId: string,
    appUrl: string,
    appPerms: string[]
  } => {
    return {
      // all these properties will have their type inferred automatically
      appId: "",
      appUrl: "",
      appPerms: []
    }
  },
  getters: {
    getAppId(): string{
      return this.appId;
    },
    getAppUrl(): string{
      return this.appUrl;
    },
    getAppPerms(): string[]{
      return this.appPerms;
    }
  },
  actions: {
    setAppId(appId: string) {
      this.appId = appId;
    },
    setAppUrl(url: string) {
      this.appUrl = url;
    },
    setAppPerms(perms: string[]) {
      this.appPerms = perms;
    }
  }
})
