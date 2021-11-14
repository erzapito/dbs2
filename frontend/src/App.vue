<script>
export default {
  name: 'App',
  data: () => ({
    loadingCounter: { value: 0},
  }),
  created() {
    window.loadingCounter = this.loadingCounter;
  },
  computed: {
    loading() {
      return window.loadingCounter.value > 0;
    }
  },
}
</script>

<template>
  <div id="app">
    <div id="nav">
        <div class="title"><router-link to="/">DBS</router-link></div>
        <ul flex>
          <li><router-link to="/series">Series</router-link></li>
          <li><router-link to="/music">Music</router-link></li>
          <li><router-link to="/wanted">Wanted</router-link></li>
        </ul>
    </div>
    <div class="loading" v-show="loading">
      <img src="@/assets/loading.gif" />
    </div>
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

    ul {
        display: flex;
        width: 100%;
        height: 100%;
        justify-content: space-evenly;

        li {
            display: flex;
            height: 100%;
            align-items: center;
            justify-content: center;
        }

    }

    a {
      font-weight: bold;
      color: #2c3e50;
    }

    a.router-link-exact-active {
      color: #42b983;
    }

}
</style>
