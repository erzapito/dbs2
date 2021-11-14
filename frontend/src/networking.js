import Vue from 'vue';
import axios from 'axios';
import HomeCtrl from './controllers/home';
import SeriesCtrl from './controllers/series.vue';
import MusicCtrl from './controllers/music.vue';
import WantedCtrl from './controllers/wanted.vue';

axios.defaults.baseURL = process.env.BASE_URL;

axios.interceptors.request.use(function (config) {
    // Do something before request is sent
    if (window.loadingCounter) {
      window.loadingCounter.value++;
    }
    return config;
  }, function (error) {
    // Do something with request error
    return Promise.reject(error);
  });

// Add a response interceptor
axios.interceptors.response.use(function (response) {
    // Any status code that lie within the range of 2xx cause this function to trigger
    // Do something with response data
    window.loadingCounter.value--;
    return response;
  }, function (error) {
    // Any status codes that falls outside the range of 2xx cause this function to trigger
    // Do something with response error
    return Promise.reject(error);
  });

const networking = new Vue({
  routes: [
    { path: '/', component: HomeCtrl },
    { path: '/series', name: 'SeriesCtrl', component: SeriesCtrl },
    { path: '/music', component: MusicCtrl },
    { path: '/wanted', component: WantedCtrl },
  ],
});

export default networking;
