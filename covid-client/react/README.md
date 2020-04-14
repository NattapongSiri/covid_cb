# Covid-client built by React
This application is built by "create-react-app" cli provided by React.

It is mandatory to create `.env` file on this directory.
It shall contains following line inside it.
```
REACT_APP_APIKEY=<IBM Cloud function API key>
REACT_APP_CREATE_SESSION_ENDPOINT=<Endpoint URL for create Watson Assistant session>
REACT_APP_SEND_MSG_ENDPOINT=<Endpoint URL to send user input message to Watson Assistant>

REACT_APP_RETRY_SEND=<Number of retry per single user input in case of send failure>
```
On each retry, it'll attempt to create a new Watson Assistant session.

# Project structure
```
react
|-build         A directory contains optimized built. It store result of npm build command
|-public        A directory contains static asset such as index.html
|-src           A directory contains React related assets.
  |-components  A components used by this project
  |-functions   A core functions used by this project
  |-messages    A directory that will store localization message for UI in the future
  |-pages       A directory that assemble components together to form a single unit of page
```

## Components
Inside components directory, there's some components that rely on each other.
The three major components are
1. Header               Contains LightSwitch and Language selector
1. ChatInput            Input for user, i.e. Keyboard and mic
    1. Speech2Text      Speech to text component that render icon and return recognized text
    1. Spinner          A three dotted spinning animation
1. ChatDialog           A dialog that contains messages and parse response from server
    1.1 ChatMessage     A bubble contains text/preview and avatar of user

# How to build
## Development build
1. npm install
1. npm start
1. Edit code and it'll auto deploy to dev server
## Production build
1. npm install (Required only if you haven't done it on development build)
1. npm build
1. built code will be put on sub-directory `build`