# msg-wa
A cloud function that take following parameters:
1. context - JSON object that will be taken into Watson Assistant "main skill" context.
1. message - Plain text string message
1. sessionId - Watson Assistant's session id. This can be obtain from `start-wa` cloud function
1. sourceLang - Two character ISO-639-1 language code. See Watson Language Translation list of supported language for complete list usable language code.
1. userid - This is reserved for future use
It return entire Watson Assistant response object.

# How to build
See parent project on how to build