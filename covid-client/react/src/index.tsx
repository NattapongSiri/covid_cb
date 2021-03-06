import React from 'react';
import ReactDOM from 'react-dom';
import i18n from 'i18next'
import {initReactI18next} from 'react-i18next'
import './index.css';
import App from './App';
import * as serviceWorker from './serviceWorker';
import messages from './messages'

let defaultLocale = navigator.language?navigator.language:"en_US"
i18n.use(initReactI18next)
    .init({
      resources: messages,
      lng: defaultLocale
    })

ReactDOM.render(
  <React.StrictMode>
      <App />
  </React.StrictMode>,
  document.getElementById('root')
);

// If you want your app to work offline and load faster, you can change
// unregister() to register() below. Note this comes with some pitfalls.
// Learn more about service workers: https://bit.ly/CRA-PWA
serviceWorker.unregister();
