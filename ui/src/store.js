/* eslint-disable */

import Vue from 'vue';
import Vuex from 'vuex';

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    socket: {
      isConnected: false,
      message: '',
    },
    feed: [],
    sequence: 0,
  },
  mutations:{
    SOCKET_ONOPEN (state, event)  {
      state.socket.isConnected = true;
    },
    SOCKET_ONCLOSE (state, event)  {
      state.socket.isConnected = false;
    },
    SOCKET_ONERROR (state, event)  {
      console.error(state, event);
    },
    // default handler called for all methods
    SOCKET_ONMESSAGE (state, message)  {
      state.message = message;
      state.feed.push({
        odometer: state.sequence,
        ...message
      });
      state.sequence++;
    }
  }
})