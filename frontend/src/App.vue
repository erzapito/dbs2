<script>
export default {
  name: 'App',
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
    <router-view/>
  </div>
</template>

<style lang="scss">
@import './assets/reset.css';

html {
    padding: 0px;
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
