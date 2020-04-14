# Covid chatbot project
A root directory contains sub-project that assemble into multi-language chatbot.
It leverage IBM Cloud Functions, Watson Language Translator, and Watson Assistant.
It can also indirectly leverage Watson Discovery to provide "Search skill" for Watson Assistant for some specific input of user.

# Main project structure
- `covid-client` - A client app that interact directly with user
    - `react` - Currently only client available
- `covid-wa-gateway` - A bunch of cloud functions that act as gateway for Watson Assistant API
    - `msg-wa` - A gateway function that forward message to Watson Assistant
    - `start-wa` - A gateway function that return new sessionId
    - `stop-wa` - A gateway function that delete session
- `covid-wlt-gateway` - A bunch of cloud functions that perform machine translate
    - `input-translate` - Translate user input and return exactly same structure as input
    - `output-translate` - Translate Watson Assistant output into user language
- `wlt-dataset` - A dataset that shall improve Watson Language Translate result thus enhance Watson Asistant dialog performance.