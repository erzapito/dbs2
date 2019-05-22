import Vue from 'vue'
import Router from 'vue-router'
import HomeCtrl from "../controllers/home";
import SeriesCtrl from "../controllers/series.vue";
import MusicCtrl from "../controllers/music";
import BooksCtrl from "../controllers/books";
import WantedCtrl from "../controllers/wanted";

Vue.use(Router)

export default new Router({
  routes: [
    { path: '/', component: HomeCtrl },
    { path: '/series', name: 'series', component: SeriesCtrl },
    { path: '/music', component: MusicCtrl },
    { path: '/books', component: BooksCtrl },
    { path: '/wanted', component: WantedCtrl },
  ]
});
