<script>
import loader from './components/loader.vue';
import axios from 'axios';

export default {
  name: 'App',
  components: {
    loader,
  },
  data: () => ({
    isLoading: false,
  }),
  mounted: function() {
    this.enableInterceptor();
  },
  methods: {
    enableInterceptor() {
        this.axiosInterceptor = axios.interceptors.request.use((config) => {
            this.isLoading = true;
            return config;
        }, (error) => {
            this.isLoading = false;
            return Promise.reject(error);
        })
        
        axios.interceptors.response.use((response) => {
            this.isLoading = false;
            return response;
        }, function(error) {
            this.isLoading = false;
            return Promise.reject(error);
        })
    },

    disableInterceptor() {
        axios.interceptors.request.eject(this.axiosInterceptor);
    },

  },
}
</script>

<template>
  <div id="app">
    <div id="nav">
        <div class="title"><h1><router-link to="/">DBS</router-link></h1></div>
        <router-link class="nav-link" to="/series">Series</router-link>
        <router-link class="nav-link" to="/music">Music</router-link>
        <router-link class="nav-link" to="/wanted">Wanted</router-link>
    </div>
    <loader :is-visible="isLoading"></loader>
    <router-view/>
  </div>
</template>

<style lang="scss">
@import './assets/reset.css';

html {
    padding: 0px;
}

.form-container {
  position: absolute;
  width: 100vw;
  height: 100vh;
  background: #0008;
  top: 0px;

  .dialog-form {
    position: relative;
    background: #BFB;
    top: calc( 50vh - 75px );
    border-radius: 15px;
    padding: 20px;
    width: 300px;
    margin: auto;
  }
}

.loading {
  position: absolute;
  width: 100vw;
  height: 100vh;
  background: #0008;
  top: 0;
  
  img {
    width: 32px;
    height: 32px;
    top: calc( 50vh - 16px );
    position: relative;
  }
}

#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}

#nav {
    width: 100%;
    height: 50px;
    background: #CCCCCC;
    display: flex;

    .title {
        display: flex;
        background: #EEEEEE;
        width: 150px;
        align-items: center;
        justify-content: center;
    }

    .nav-link {
      display: flex;
      height: 100%;
      align-items: center;
      justify-content: center;
      padding-left: 20px;
      padding-right: 20px;

      &.router-link-exact-active {
        color: #EEEEEE;
        background: #AAAAAA;
      }

    }

}
</style>
