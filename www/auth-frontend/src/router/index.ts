import { createRouter, createWebHistory } from 'vue-router'
import AuthView from '../views/AuthView.vue'
import HomeView from '@/views/HomeView.vue'
import NotFoundView from '@/views/NotFoundView.vue'
import LoginView from '@/views/LoginView.vue'
import RegisterView from '@/views/RegisterView.vue'
import MainPanelView from '@/views/panel/MainPanelView.vue'
import AppsPanelView from '@/views/panel/AppsPanelView.vue'
import AppPanelView from '@/views/panel/AppPanelView.vue'
import UsersPanelView from '@/views/panel/UsersPanelView.vue'
import UserPanelView from '@/views/panel/UserPanelView.vue'
import LogoutView from '@/views/LogoutView.vue'
import AuthPanelView from '@/views/panel/AuthPanelView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },
    {
      path: '/404',
      name: 'notfound',
      component: NotFoundView
    },
    {
      path: '/login',
      name: 'login',
      component: LoginView
    },
    {
      path: '/register',
      name: 'register',
      component: RegisterView
    },
    {
      path: '/auth',
      name: 'auth',
      component: AuthView,
    },
    {
      path: '/logout',
      name: 'logout',
      component: LogoutView
    },
    {
      path: '/panel',
      name: 'panel',
      component: MainPanelView,
      children: [
        {
          path: '/panel/apps',
          name: 'panelapps',
          component: AppsPanelView
        },
        {
          path: '/panel/app',
          name: 'panelapp',
          component: AppPanelView
        },
        {
          path: '/panel/users',
          name: 'panelusers',
          component: UsersPanelView
        },
        {
          path: '/panel/user',
          name: 'paneluser',
          component: UserPanelView
        },
        {
          path: '/panel/auth',
          name: 'panelauth',
          component: AuthPanelView
        }
      ]
    }
  ],
});

export default router
