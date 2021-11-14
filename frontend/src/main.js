import Vue from 'vue';
import App from './App.vue';
import router from './router';
import networking from './networking';

Vue.config.productionTip = false;

new Vue({
  router,
  networking,
  render: h => h(App)
}).$mount('#app')
