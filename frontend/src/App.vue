<template>
  <div id="app">
    <div id="nav">
        <ul>
          <li><router-link to="/series">Series</router-link></li>
          <li><router-link to="/music">Music</router-link></li>
          <li><router-link to="/wanted">Wanted</router-link></li>
        </ul>
    </div>
    <router-view/>
  </div>
</template>

<style lang="scss">
#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}

#nav {
    padding: 30px;

    a {
      font-weight: bold;
      color: #2c3e50;
    }

    a.router-link-exact-active {
      color: #42b983;
    }

}
</style>
