import { createApp } from "vue";
import App from "./App.vue";
import { createI18n } from 'vue-i18n'
import '../node_modules/@thatzokay/vue-aplayer/dist/scss/vue-aplayer.scss';
import "./assets/main.css";
import router from "./router";

import { OhVueIcon, addIcons } from "oh-vue-icons";
import { GiSubmarine } from "oh-vue-icons/icons";

import * as enUS from './assets/locales/en/translation.json';
import * as nlNL from './assets/locales/nl/translation.json';
import Toast, { PluginOptions } from "vue-toastification";
import { VueAPlayerPlugin } from '@thatzokay/vue-aplayer';

import "vue-toastification/src/scss/index.scss";

const i18n = createI18n({
  legacy: false,
  locale: window.navigator.language,
  fallbackLocale: 'en-US',
  messages: {
    'en-US': {
      ...enUS
    },
    'en': {
      ...enUS
    },
    'nl-NL': {
      ...nlNL
    },
    'nl': {
      ...nlNL
    }
  }
})

addIcons(GiSubmarine);

const app = createApp(App)
  .use(router);

const options: PluginOptions = {
  // You can set your default options here
};

router.isReady().then(() => {
  app.use(i18n);
  app.use(Toast, options);
  app.use(VueAPlayerPlugin);
  app.component("v-icon", OhVueIcon);
  app.mount("#app");
});