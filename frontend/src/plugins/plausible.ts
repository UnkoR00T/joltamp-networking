import {type App, inject} from 'vue'
import Plausible from 'plausible-tracker'

let plausible: any = null

function setupPlausible(app: App<Element>) {
  if (import.meta.env.PROD) {
    plausible = Plausible({
      domain: 'sso.joltamp.pl',
      apiHost: 'https://plausible.joltamp.pl'
    })
    plausible.enableAutoPageviews()
    plausible.enableAutoOutboundTracking()
  }

  app.config.globalProperties.$plausible = plausible
}

function usePlausible() {
  return plausible
}

export {setupPlausible, usePlausible}
