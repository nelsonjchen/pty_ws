// The Vue build version to load with the `import` command
// (runtime-only or standalone) has been set in webpack.base.conf with an alias.
import Vue from 'vue';
import VueNativeSock from 'vue-native-websocket';
import App from './App';
import store from './store';

Vue.config.productionTip = false;

Vue.use(VueNativeSock, 'ws://localhost:8000/ws', { store, format: 'json' });

/* eslint-disable no-new */
new Vue({
  el: '#app',
  template: '<App/>',
  components: { App },
});
