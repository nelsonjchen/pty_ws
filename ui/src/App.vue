<template>
  <div id="app">
    <ul>
      <li v-for="data in feed" :key="data.sequence">
        {{ data }}
      </li>
    </ul>

  </div>
</template>

<script>

export default {
  name: 'app',
  computed: {
    feed() {
      return this.$store.state.feed;
    },
  },
  components: {},
};
</script>

<style>
#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
