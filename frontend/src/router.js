import Vue from 'vue';
import VueRouter from 'vue-router';
import HomeCtrl from './controllers/home';
import SeriesCtrl from './controllers/series.vue';
import MusicCtrl from './controllers/music.vue';
import WantedCtrl from './controllers/wanted.vue';

Vue.use(VueRouter);

const router = new VueRouter({
  routes: [
    { path: '/', component: HomeCtrl },
    { path: '/series', name: 'SeriesCtrl', component: SeriesCtrl },
    { path: '/music', component: MusicCtrl },
    { path: '/wanted', component: WantedCtrl },
  ],
});

export default router;
